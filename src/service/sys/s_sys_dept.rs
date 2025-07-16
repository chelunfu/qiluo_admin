use crate::model::sys::model::msys_dept::{
    DeptTree, DeptTreeData, SysDeptAdd, SysDeptDel, SysDeptEdit, SysDeptModel, SysDeptSearch,
};
use crate::service::prelude::*;
use std::collections::VecDeque;

pub async fn list_tree(
    VQuery(arg): VQuery<PageParams>,
    VQuery(search): VQuery<SysDeptSearch>,
) -> impl IntoResponse {
    let rlist = SysDeptModel::list_tree(arg, search).await;
    let data = match rlist {
        Ok(data) => data,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    let mut list = Vec::new();
    for dept in data.list {
        let dept_tree = DeptTree {
            dept,
            ..Default::default()
        };
        list.push(dept_tree)
    }
    let dept_trees = dept_to_tree(list, 0);
    let res = ListData {
        list: dept_trees,
        total: data.total,
        total_pages: data.total_pages,
        page_num: data.page_num,
    };
    ApiResponse::ok(res)
}

pub async fn dept_tree() -> impl IntoResponse {
    let rlist = SysDeptModel::get_all().await;
    let depts = match rlist {
        Ok(depts) => depts,
        Err(e) => return ApiResponse::bad_request(e.to_string()),
    };
    let mut list = Vec::new();
    for dept in depts {
        let dept_tree = DeptTree {
            dept,
            ..Default::default()
        };
        list.push(dept_tree)
    }
    let dept_trees = dept_to_tree(list, 0);
    let j = json!({
        "list":&dept_trees,
        "total": dept_trees.len()
    });
    ApiResponse::ok(j)
}

pub async fn edit(VJson(arg): VJson<SysDeptEdit>) -> impl IntoResponse {
    let r = SysDeptModel::edit(arg).await;
    match r {
        Ok((_, update)) => {
            if update {
                let a = update_tree().await;
                ApiResponse::from_result(a)
            } else {
                ApiResponse::ok("ok")
            }
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}
pub async fn add(VJson(arg): VJson<SysDeptAdd>) -> impl IntoResponse {
    let r = SysDeptModel::add(arg).await;
    match r {
        Ok(_) => {
            let a = update_tree().await;
            ApiResponse::ok(a)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}
pub async fn delete(VQuery(arg): VQuery<SysDeptDel>) -> impl IntoResponse {
    let r = SysDeptModel::delete(arg.dept_id).await;
    match r {
        Ok(_) => {
            let a = update_tree().await;
            ApiResponse::<String>::from_result(a)
        }
        Err(e) => ApiResponse::bad_request(e.to_string()),
    }
}
fn dept_to_tree(depts: Vec<DeptTree>, pid: i64) -> Vec<DeptTree> {
    let mut dept_trees = Vec::new();
    for mut dept in depts.clone() {
        if dept.dept.parent_id == pid {
            dept.children = Some(dept_to_tree(depts.clone(), dept.dept.dept_id));
            dept_trees.push(dept);
        }
    }
    dept_trees
}

///计算树结构
async fn update_tree() -> Result<String> {
    let rlist = SysDeptModel::tree_data_all().await;
    let depts = if let Ok(depts) = rlist {
        depts
    } else {
        return Err("Err".into());
    };
    let mut list = Vec::new();
    for dept in depts {
        let dept_tree = DeptTreeData {
            dept_id: dept.dept_id,
            parent_id: dept.parent_id,
            lft: dept.lft,
            rgt: dept.rgt,
            depth: dept.depth,
            ..Default::default()
        };
        list.push(dept_tree)
    }
    let dept_trees = data_tree(list, 0);
    for mut de in dept_trees {
        let mut counte = 1;
        calculate_left_right_values_recursive(&mut de, &mut counte, &mut 1);

        let sava = flatten_tree(&de);
        let _ = SysDeptModel::update_tree(sava).await;
    }
    Ok("Success".to_string())
}

fn calculate_left_right_values_recursive(
    dept_tree: &mut DeptTreeData,
    count: &mut i32,
    depth: &mut i32,
) {
    dept_tree.lft = *count;
    *count += 1;
    dept_tree.depth = *depth;
    if let Some(ref mut children) = dept_tree.children {
        *depth += 1;
        for child in children.iter_mut() {
            calculate_left_right_values_recursive(child, count, depth);
        }
        *depth -= 1;
    }
    dept_tree.rgt = *count;
    *count += 1;
}

fn data_tree(depts: Vec<DeptTreeData>, pid: i64) -> Vec<DeptTreeData> {
    let mut dept_trees = Vec::new();
    for mut dept in depts.clone() {
        if dept.parent_id == pid {
            dept.children = Some(data_tree(depts.clone(), dept.dept_id));
            dept_trees.push(dept);
        }
    }
    dept_trees
}

fn flatten_tree(root: &DeptTreeData) -> Vec<DeptTreeData> {
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    while let Some(node) = queue.pop_front() {
        result.push(node.clone());
        if let Some(ref children) = node.children {
            for child in children {
                queue.push_back(child.clone());
            }
        }
    }
    result
}
