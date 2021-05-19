use fluminurs::module::Module;
use fluminurs::Api;

use crate::pages::files::FilesMessage;
use crate::pages::login::LoginMessage;
use crate::pages::modules::ModulesMessage;
use crate::pages::Page;
use crate::Error;

#[derive(Debug)]
pub enum Message {
    LoginMessage(LoginMessage),
    ModulesMessage(ModulesMessage),
    FilesMessage(FilesMessage),
    SwitchPage(Page),
    LoadedAPI(Result<(Api, String, Vec<Module>, Vec<String>), Error>),
}
