use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{
    filemap::FileMap,
    prelexer::PreLexer,
    utils::{pretoken::PreToken, structs::FilePreTokPos},
};

#[derive(Debug)]
struct FileLexer {
    pub compFile: String,
    pub lexer: PreLexer,
}

#[derive(Debug)]
pub struct MultiLexer {
    fileMapping: Arc<Mutex<FileMap>>,
    files: Vec<FileLexer>,
    pushedTokens: VecDeque<FilePreTokPos<PreToken>>,
}

impl MultiLexer {
    pub fn new_def(files: Arc<Mutex<FileMap>>) -> MultiLexer {
        MultiLexer {
            fileMapping: files,
            files: vec![],
            pushedTokens: VecDeque::new(),
        }
    }

    pub fn new((files, file): (Arc<Mutex<FileMap>>, &str)) -> MultiLexer {
        let lexer = {
            let currFile = files.lock().unwrap().getFile(file);
            PreLexer::new(currFile.content().clone())
        };

        MultiLexer {
            fileMapping: files,
            files: vec![FileLexer {
                compFile: file.to_string(),
                lexer: lexer,
            }],
            pushedTokens: VecDeque::new(),
        }
    }

    pub fn pushTokensDec(&mut self, mut toks: VecDeque<FilePreTokPos<PreToken>>) -> () {
        self.pushedTokens.append(&mut toks);
    }

    pub fn pushTokensVec(&mut self, toks: Vec<FilePreTokPos<PreToken>>) -> () {
        self.pushedTokens.append(
            &mut toks
                .into_iter()
                .collect::<VecDeque<FilePreTokPos<PreToken>>>(),
        );
    }

    pub fn pushToken(&mut self, tok: FilePreTokPos<PreToken>) -> () {
        self.pushedTokens.push_back(tok);
    }

    pub fn pushFile(&mut self, path: String) -> () {
        self.files.push(FileLexer {
            compFile: path.clone(),
            lexer: PreLexer::new(
                self.fileMapping
                    .lock()
                    .unwrap()
                    .getAddFile(path.as_str())
                    .content()
                    .to_string(),
            ),
        });
    }

    pub fn expectHeader(&mut self) {
        if let Some(lex) = self.files.last_mut() {
            lex.lexer.expectHeader();
        }
    }

    pub fn fileMapping(&self) -> Arc<Mutex<FileMap>> {
        self.fileMapping.clone()
    }
}

impl Iterator for MultiLexer {
    type Item = FilePreTokPos<PreToken>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.pushedTokens.pop_front() {
            return Some(t);
        }
        loop {
            let mut popLexer = false;
            if let Some(lexer) = self.files.last_mut() {
                match lexer.lexer.next() {
                    None => {
                        popLexer = true;
                    }
                    Some(tok) => {
                        return Some(FilePreTokPos::new(
                            self.fileMapping.lock().unwrap().getFile(&lexer.compFile),
                            tok,
                        ));
                    }
                }
            } else {
                return None;
            }

            if popLexer {
                self.files.pop();
            }
        }
    }
}
