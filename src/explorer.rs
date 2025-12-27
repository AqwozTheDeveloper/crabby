use crate::manifest::{CrabbyLock, PackageJson};


pub fn find_dependency_paths(lock: &CrabbyLock, pkg: &PackageJson, target: &str) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    
    // Start from direct dependencies
    for dep in pkg.dependencies.keys() {
        let mut current_path = vec![dep.clone()];
        if dep == target {
            paths.push(current_path.clone());
        } else {
            search_recursive(lock, dep, target, &mut current_path, &mut paths);
        }
    }

    // Start from dev dependencies
    for dep in pkg.dev_dependencies.keys() {
        let mut current_path = vec![format!("{} (dev)", dep)];
        if dep == target {
            paths.push(current_path.clone());
        } else {
            search_recursive(lock, dep, target, &mut current_path, &mut paths);
        }
    }

    paths
}

fn search_recursive(lock: &CrabbyLock, current: &str, target: &str, path: &mut Vec<String>, results: &mut Vec<Vec<String>>) {
    // Avoid cycles
    if path.len() > 10 { return; }

    if let Some(dep_info) = lock.dependencies.get(current) {
        for sub_dep in dep_info.dependencies.keys() {
            // Check for cycles in path
            if path.contains(sub_dep) { continue; }

            path.push(sub_dep.clone());
            if sub_dep == target {
                results.push(path.clone());
            } else {
                search_recursive(lock, sub_dep, target, path, results);
            }
            path.pop();
        }
    }
}
