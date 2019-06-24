use crate::commands::command::SinkCommandArgs;
use crate::errors::ShellError;
use crate::format::{GenericView, TableView};
use crate::prelude::*;

pub fn autoview(args: SinkCommandArgs) -> Result<(), ShellError> {
    if args.input.len() > 0 {
        if equal_shapes(&args.input) {
            let mut host = args.ctx.host.lock().unwrap();
            let view = TableView::from_list(&args.input).unwrap();

            handle_unexpected(&mut *host, |host| crate::format::print_view(&view, host));
        } else {
            let mut host = args.ctx.host.lock().unwrap();
            for i in args.input.iter() {
                let view = GenericView::new(&i);
                handle_unexpected(&mut *host, |host| crate::format::print_view(&view, host));
                host.stdout("");
            }
        }
    }

    Ok(())
}

fn equal_shapes(input: &Vec<Value>) -> bool {
    let mut items = input.iter();

    let item = match items.next() {
        Some(item) => item,
        None => return false,
    };

    let desc = item.data_descriptors();

    for item in items {
        if desc != item.data_descriptors() {
            return false;
        }
    }

    true
}