
use crate::*;

impl CommandService for Hget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get(&self.table, &self.key) {
            Ok(Some(v)) => v.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let mut vs = vec![];
        for x in self.pairs {
            match store.set(&self.table,x.key,x.value.unwrap_or_default()) {
                Ok(Some(v)) => {
                    vs.push(v);
                },
                Ok(None) => {
                    vs.push(Value{
                        value:None
                    });
                },
                Err(e) => {
                    return e.into();
                },
            }
        }
        vs.into()
    }
}

impl CommandService for Hmget {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        let mut vs = vec![];
        for ref x in self.keys {
            match store.get(&self.table,x) {
                Ok(Some(v)) => {
                    vs.push(v);
                },
                Ok(None) => {
                    vs.push(Value{
                        value: None
                    });
                },
                Err(e) => return e.into(),
            }
        }
        vs.into()
    }
}

impl CommandService for Hgetall {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match store.get_all(&self.table) {
            Ok(v) => v.into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute(self, store: &impl Storage) -> CommandResponse {
        match self.pair {
            Some(v) => match store.set(&self.table, v.key, v.value.unwrap_or_default()) {
                Ok(Some(v)) => v.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => KvError::InvalidCommand(format!("{:?}", self)).into(),
        }
    }
}