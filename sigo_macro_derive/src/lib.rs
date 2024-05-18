use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// REVIEW: maybe these derive should be Trait
#[proc_macro_derive(FiledTask)]
pub fn filedtask_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate_filedtask(input) {
        Ok(ts) => ts,
        Err(err) => panic!("{}", err),
    }
}

#[proc_macro_derive(IdAssignedTask)]
pub fn idassignedtask_derive(input: TokenStream) -> TokenStream {
    let input = &parse_macro_input!(input as DeriveInput);
    match generate_idassignedtask(input) {
        Ok(ts) => ts,
        Err(err) => panic!("{}", err),
    }
}

fn generate_filedtask(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    // return Err if input is not struct
    match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Must be struct type",
            ));
        }
    };

    let struct_name = &derive_input.ident;
    let expanded = quote! {
        impl #struct_name {
            pub fn read_tasks(cfg: &MyConfig) -> Result<Vec<Self>, SigoError> {
                let mut path = PathBuf::from(&cfg.data);
                path.push(Self::FILE_NAME);
                utils::create_file_if_not_exist(&path)?;
                let tasks = std::fs::read_to_string(path.clone())
                    .map_err(|e| SigoError::FileReadErr(path.clone(), e))?;
                let tasks = serde_json::from_str::<Vec<Self>>(&tasks)
                    .map_err(|e| SigoError::ParseStrToTasksErr(path.clone(), e))?;
                Ok(tasks)
            }

            pub fn write_tasks(cfg: &MyConfig, tasks: Vec<Self>) -> Result<(), SigoError> {
                let mut path = PathBuf::from(&cfg.data);
                path.push(Self::FILE_NAME);
                utils::create_file_if_not_exist(&path)?;
                let tmp_path = path.with_extension(format!("sigo-tmp-{}", std::process::id()));
                let mut file = std::fs::File::create(&tmp_path)
                    .map_err(|e| SigoError::FileCreateErr(tmp_path.clone(), e))?;
                let tasks = serde_json::to_string(&tasks)?;
                std::io::BufWriter::with_capacity(tasks.len(), &file)
                    .write_all(tasks.as_bytes())
                    .map_err(|e| SigoError::FileWriteErr(tmp_path.clone(), e))?;
                file.flush()
                    .map_err(|e| SigoError::FileWriteErr(tmp_path.clone(), e))?;
                std::fs::rename(&tmp_path, &path)
                    .map_err(|e| SigoError::FileRenameErr(tmp_path.clone(), path.clone(), e))?;
                Ok(())
            }

            pub fn add_task(cfg: &MyConfig, task: Self) -> Result<Self, SigoError> {
                let mut tasks = Self::read_tasks(cfg)?;
                tasks.push(task.clone());
                Self::write_tasks(cfg, tasks)?;
                Ok(task)
            }
        }
    };

    Ok(expanded.into())
}

fn generate_idassignedtask(derive_input: &DeriveInput) -> Result<TokenStream, syn::Error> {
    // return Err if input is not struct
    match &derive_input.data {
        syn::Data::Struct(v) => v,
        _ => {
            return Err(syn::Error::new_spanned(
                &derive_input.ident,
                "Must be struct type",
            ));
        }
    };

    let struct_name = &derive_input.ident;
    let expanded = quote! {
        impl #struct_name {
            fn get_by_id(cfg: &MyConfig, id: u32) -> Result<Self, SigoError> {
                let tasks = Self::read_tasks(cfg)?;
                tasks
                    .into_iter()
                    .find(|t| t.id == id)
                    .ok_or(SigoError::TaskNotFound(id))
            }

            fn delete_by_id(cfg: &MyConfig, id: u32) -> Result<(), SigoError> {
                let tasks = Self::read_tasks(cfg)?;
                let updated_tasks = tasks
                    .into_iter()
                    .filter(|t| t.id != id)
                    .collect::<Vec<Self>>();
                Self::write_tasks(cfg, updated_tasks)?;
                Ok(())
            }

            pub fn get_main_description(&self) -> String {
                match &self.description {
                    Some(v) => v
                        .get(0)
                        .unwrap_or(&"No description".to_string())
                        .to_string(),
                    None => "No description".to_string(),
                }
            }

            pub fn complete(&self, cfg: &MyConfig) -> Result<CompletedTask, SigoError> {
                let before_tasks = Self::read_tasks(cfg)?;
                let after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != self.id)
                    .collect::<Vec<Self>>();
                Self::write_tasks(cfg, after_tasks)?;
                let completed_task = CompletedTask {
                    description: <std::option::Option<Vec<std::string::String>> as Clone>::clone(
                        &self.description,
                    )
                    .unwrap_or_default()
                    .concat(),
                };
                let mut completed_tasks = CompletedTask::read_tasks(cfg)?;
                CompletedTask::add_task(cfg, completed_task.clone())?;
                Ok(completed_task)
            }

            pub fn annotate(&self, cfg: &MyConfig, annotate: &str) -> Result<(), SigoError> {
                let id = self.id;
                let before_tasks = Self::read_tasks(cfg)?;
                let mut after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != id)
                    .collect::<Vec<Self>>();
                let mut description =
                    <std::option::Option<Vec<std::string::String>> as Clone>::clone(
                        &self.description,
                    )
                    .unwrap_or_default();
                description.push(annotate.to_owned());
                let annotated_task = Self {
                    id: self.id,
                    description: Some(description),
                    priority: self.priority,
                };
                after_tasks.push(annotated_task);
                Self::write_tasks(cfg, after_tasks)?;
                Ok(())
            }

        pub fn modify(
            &self,
            cfg: &MyConfig,
            text: &Option<String>,
            priority: Option<Priority>,
        ) -> Result<Self, SigoError> {
                let id = self.id;
                let before_tasks = Self::read_tasks(cfg)?;
                let mut after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != id)
                    .collect::<Vec<Self>>();
                let mut description =
                    <std::option::Option<Vec<std::string::String>> as Clone>::clone(
                        &self.description,
                    )
                    .unwrap_or_default();
                if let Some(text) = text {
                    if let Some(first_description) = description.get_mut(0) {
                        *first_description = text.to_string()
                    }
                }
                let new_task = Self {
                    id: self.id,
                    description: Some(description),
                    priority: priority.or(self.priority),
                };
                after_tasks.push(new_task.clone());
                Self::write_tasks(cfg, after_tasks)?;
            Ok(new_task)
        }
    }

    };

    Ok(expanded.into())
}
