use fluminurs::module::Module;
use fluminurs::file::File;
use fluminurs::Api;

use crate::header::HeaderMessage;
use crate::pages::files::FilesMessage;
use crate::pages::login::LoginMessage;
use crate::pages::modules::ModulesMessage;
use crate::pages::Page;
use crate::Error;

#[derive(Debug)]
pub enum Message {
    LoginPage(LoginMessage),
    ModulesPage(ModulesMessage),
    FilesPage(FilesMessage),
    Header(HeaderMessage),
    SwitchPage(Page),
    LoadedAPI(Result<(Api, String, Vec<Module>), Error>),
    LoadedFiles(Result<Vec<File>, Error>),
}
