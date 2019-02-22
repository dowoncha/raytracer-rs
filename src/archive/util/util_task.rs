pub type TaskFn = Fn(i32);

struct TaskPool {
    num: i32,
    do_cancel: bool,
    /* time time stamp of first task pushed. */
    start_time: f64,

    /* Number of all tasks handled by this pool. */
    num_tasks_handled: i32
}

struct Task {
}

impl TaskPool {
    pub fn new() -> TaskPool {
        TaskPool {
            num_tasks_handled: 0,
            num: 0,
            start_time: 0.0,
            do_cancel: false,
        }
    }

    pub fn push_task(task: Box<Task>, front: bool) {

    }

}

struct TaskScheduler {
}

