use super::toast;
use super::tr::tr;
use crate::db;
use crate::db::def::{TODO_TABLE, TodoEntry};
use crate::slint_generatedAppWindow::{AppWindow, Logic, Store, TodoEntry as UITodoEntry};
use slint::{ComponentHandle, Model, VecModel, Weak};
use uuid::Uuid;

#[macro_export]
macro_rules! store_todo_entries {
    ($ui:expr) => {
        $ui.global::<Store>()
            .get_todo_entries()
            .as_any()
            .downcast_ref::<VecModel<UITodoEntry>>()
            .expect("We know we set a VecModel earlier")
    };
}

async fn get_from_db() -> Vec<UITodoEntry> {
    let mut entries = match db::entry::select_all(TODO_TABLE).await {
        Ok(items) => items
            .into_iter()
            .filter_map(|item| serde_json::from_str::<TodoEntry>(&item.data).ok())
            .map(|item| item.into())
            .collect(),
        Err(e) => {
            log::warn!("{:?}", e);
            vec![]
        }
    };

    entries.sort_by(|a: &UITodoEntry, b: &UITodoEntry| b.priority.cmp(&a.priority));

    entries
}

fn todo_init(ui: Weak<AppWindow>) {
    tokio::spawn(async move {
        let entries = get_from_db().await;

        let _ = slint::invoke_from_event_loop(move || {
            store_todo_entries!(ui.unwrap()).set_vec(entries);
        });
    });
}

#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
pub fn init(ui: &AppWindow) {
    todo_init(ui.as_weak());

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_todo_update(move |mut entry| {
        let ui = ui_handle.unwrap();

        if entry.uuid.is_empty() {
            entry.uuid = Uuid::new_v4().to_string().into();
            add_entry(&ui, entry);
        } else {
            update_entry(&ui, entry);
        }
    });

    let ui_handle = ui.as_weak();
    ui.global::<Logic>().on_todo_delete(move |uuid| {
        let ui = ui_handle.unwrap();

        for (index, entry) in store_todo_entries!(ui).iter().enumerate() {
            if entry.uuid != uuid {
                continue;
            }

            store_todo_entries!(ui).remove(index);

            let ui = ui.as_weak();
            tokio::spawn(async move {
                match db::entry::delete(TODO_TABLE, uuid.as_str()).await {
                    Err(e) => toast::async_toast_warn(
                        ui,
                        format!("{}. {}: {e:?}", tr("Remove entry failed"), tr("Reason")),
                    ),
                    _ => toast::async_toast_success(ui, tr("Remove entry successfully")),
                }
            });

            return;
        }
    });
}

fn add_entry(ui: &AppWindow, entry_ui: UITodoEntry) {
    let mut is_inserted = false;
    let entry_db: TodoEntry = entry_ui.clone().into();

    for (index, entry) in store_todo_entries!(ui).iter().enumerate() {
        if entry.priority >= entry_ui.priority {
            continue;
        }

        store_todo_entries!(ui).insert(index, entry_ui.clone());
        is_inserted = true;
        break;
    }

    if !is_inserted {
        store_todo_entries!(ui).push(entry_ui);
    }

    let ui = ui.as_weak();
    tokio::spawn(async move {
        let data = serde_json::to_string(&entry_db).unwrap();
        match db::entry::insert(TODO_TABLE, &entry_db.uuid, &data).await {
            Err(e) => toast::async_toast_warn(
                ui,
                format!("{}. {}: {e:?}", tr("Add entry failed"), tr("Reason")),
            ),
            _ => toast::async_toast_success(ui, tr("Add entry successfully")),
        }
    });
}

fn update_entry(ui: &AppWindow, entry_ui: UITodoEntry) {
    let mut is_priority_modified = false;
    let entry_db: TodoEntry = entry_ui.clone().into();

    for (index, entry) in store_todo_entries!(ui).iter().enumerate() {
        if entry.uuid != entry_ui.uuid {
            continue;
        }

        if entry.priority == entry_ui.priority {
            store_todo_entries!(ui).set_row_data(index, entry_ui.clone());
        } else {
            store_todo_entries!(ui).remove(index);
            is_priority_modified = true;
        }

        break;
    }

    if is_priority_modified {
        let mut is_inserted = false;

        for (index, entry) in store_todo_entries!(ui).iter().enumerate() {
            if entry.priority >= entry_ui.priority {
                continue;
            }

            store_todo_entries!(ui).insert(index, entry_ui.clone());
            is_inserted = true;
            break;
        }

        if !is_inserted {
            store_todo_entries!(ui).push(entry_ui);
        }
    }

    let ui = ui.as_weak();
    tokio::spawn(async move {
        let data = serde_json::to_string(&entry_db).unwrap();
        match db::entry::update(TODO_TABLE, &entry_db.uuid, &data).await {
            Err(e) => toast::async_toast_warn(
                ui,
                format!("{}. {}: {e:?}", tr("Update entry failed"), tr("Reason")),
            ),
            _ => (),
        }
    });
}
