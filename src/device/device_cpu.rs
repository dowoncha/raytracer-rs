use super::{Device, DeviceInfo};
use super::task::{DeviceTask, DeviceTaskKind};

use render::{Rendertile,RenderTileTask};

use kernel;

struct DeviceCPU {

}

impl DeviceCPU {
    fn thread_run(&self, task: &DeviceTask) {
        match task.kind {
            DeviceTaskKind::Render => {
                self.thread_render(task);
            },
            DeviceTaskKind::FilmConvert => {
                unimplemented!();
            },
            DeviceTaskKind::Shader => {
                unimplemented!();
            }
        }
    }

    /**
     * Starts a new thread to render
     */
    fn thread_render(&self, task: &DeviceTask) {
        // If the task pool is canceled

        // Allocate buffer for kernel globals
        let kgbuffer = Box::new(kernel::Globals::new());
        let kg = kgbuffer;/* KernelGLobals(thread_kernel_globals_init()) */

        /* What is a split kernel? */

        while let Some(tile) = task.acquire_tile() {
            match tile.task {
                RenderTileTask::PathTrace => {
                    self.path_trace(task, tile, kg)
                },
                RenderTileTask::Denoise => {
                    // denoise(task, denoising, tile);
                }
            }

            // task.release_tile(tile);
            // if task_pool.canceled() {
            break;   
        }

        // Cleanup kgbuffer
    }

    fn path_trace(&self, task: &DeviceTask, tile: &RenderTile, kg: &KernelGlobals) {
        // timer

        let ref render_buffer = tile.buffer;
        let end_sample = tile.start_sample + tile.sample;

        for sample in tile.start_sample..end_sample {
            // Check if task/pool got cancelled

            for y in tile.y..(tile.y + tile.h) {
                for x in tile.x..(tile.x + tile.w) {
                    kernel::path_trace(
                        kg,
                        render_buffer,
                        sample,
                        x,
                        y,
                        tile.offset,
                        tile.stride
                    );
                }
            }

            tile.sample = sample + 1;
            // task.update_progress(&tile, tile.w * tile.h)
        }

    }

}

impl Device for DeviceCPU {
    fn new(info: &DeviceInfo, background: bool) -> Self {
        DeviceCPU {

        }
    }

    fn task_add(&self, task: &DeviceTask) {
        /* load texture info */

        /* split task into smaller ones */
        let split_tasks: Vec<DeviceTask> = Vec::new();

        match task.kind {
            DeviceTaskKind::Shader => {
                // task.split(tasks, info.cpu_threads, 256);
            },
            _ => { /* task.split(tasks, info.cpu_threads) */}
        }

        for task in split_tasks {
            // task_pool.push(new CPUDeviceTask(this, task));
        }
    }    

    fn task_wait(&self) {

    }

    fn task_cancel(&self) {

    }
}