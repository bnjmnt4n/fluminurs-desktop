use fluminurs::module::Module;
use fluminurs::Api;

use crate::header::HeaderMessage;
use crate::pages::files::FilesMessage;
use crate::pages::login::LoginMessage;
use crate::pages::modules::ModulesMessage;
use crate::pages::Page;
use crate::resource::ResourceMessage;
use crate::resource::ResourceState;
use crate::Error;

#[derive(Debug)]
pub enum Message {
    LoginPage(LoginMessage),
    ModulesPage(ModulesMessage),
    FilesPage(FilesMessage),
    Header(HeaderMessage),
    SwitchPage(Page),
    ResourceMessage((String, ResourceMessage)),
    LoadedAPI(Result<(Api, String, Vec<Module>), Error>),
    LoadedFiles(Result<Vec<ResourceState>, Error>),
    ResourceDownloaded(Result<String, Error>),
}
