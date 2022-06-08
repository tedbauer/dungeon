use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::mpsc::{self, Receiver};
use std::sync::Arc;
use std::thread::{self};

trait System {
    fn tick(&self);
    fn id(&self) -> u32;
}

struct UpdatePositions {}

impl System for UpdatePositions {
    fn tick(&self) {
        println!("Hello from the update positions system.");
    }

    fn id(&self) -> u32 {
        0
    }
}

struct RenderEntities {}

impl System for RenderEntities {
    fn tick(&self) {
        println!("Hello from the render entities system.");
    }

    fn id(&self) -> u32 {
        1
    }
}

struct SystemRunner {}

enum ScheduleBlock {
    Parallel(Vec<ScheduleBlock>),
    Sequence(Vec<ScheduleBlock>),
    Single(Arc<dyn System + Send + Sync + 'static>),
}

enum BlockInner {
    Parallel(Vec<BlockInner>),
    Sequence(Vec<BlockInner>),
    Single(u32),
}

type SystemStart = Sender<()>;
type SystemFinish = Receiver<()>;
type Controller = HashMap<u32, (SystemStart, SystemFinish)>;

impl SystemRunner {
    fn _execute(&self, schedule: BlockInner, controller: &Controller) -> Vec<u32> {
        match schedule {
            | BlockInner::Parallel(blocks) => {
                let mut systems = Vec::new();
                for block in blocks {
                    systems.extend(self._execute(block, controller))
                }
                systems
            }
            | BlockInner::Sequence(blocks) => {
                for block in blocks {
                    let systems_to_wait = self._execute(block, controller);
                    for system in systems_to_wait {
                        let system_fin = &controller.get(&system).unwrap().1;
                        system_fin.recv();
                    }
                }
                Vec::new()
            }
            | BlockInner::Single(system) => {
                controller.get(&system).unwrap().0.send(());
                vec![system]
            }
        }
    }

    fn transform(&self, schedule: &ScheduleBlock) -> BlockInner {
        match schedule {
            | ScheduleBlock::Parallel(blocks) => {
                BlockInner::Parallel(blocks.iter().map(|b| self.transform(b)).collect())
            }
            | ScheduleBlock::Sequence(blocks) => {
                BlockInner::Sequence(blocks.iter().map(|b| self.transform(b)).collect())
            }
            | ScheduleBlock::Single(system) => BlockInner::Single(system.id()),
        }
    }

    fn gen_sys_threads(&self, schedule: ScheduleBlock) -> Controller {
        match schedule {
            | ScheduleBlock::Single(system) => {
                let mut res: Controller = HashMap::new();
                let sys_id = system.id();

                let (send_sys_start, recv_sys_start) = mpsc::channel();
                let (send_sys_fin, recv_sys_fin) = mpsc::channel();
                thread::spawn(move || {
                    while recv_sys_start.recv() == Ok(()) {
                        system.tick();
                        send_sys_fin.send(());
                    }
                });
                res.insert(sys_id, (send_sys_start, recv_sys_fin));
                res
            }
            | ScheduleBlock::Sequence(blocks) => {
                let mut res: Controller = HashMap::new();
                for block in blocks {
                    res.extend(self.gen_sys_threads(block));
                }
                res
            }
            | ScheduleBlock::Parallel(blocks) => {
                let mut res: Controller = HashMap::new();
                for block in blocks {
                    res.extend(self.gen_sys_threads(block));
                }
                res
            }
        }
    }

    fn execute(&self, schedule: ScheduleBlock) {
        let transformed = self.transform(&schedule);
        let controller = self.gen_sys_threads(schedule);
        self._execute(transformed, &controller);
    }
}

pub fn main() {
    println!("Hello.");

    let update_positions = UpdatePositions {};
    let render_entities = RenderEntities {};
    let runner = SystemRunner {};

    let schedule: ScheduleBlock = ScheduleBlock::Sequence(vec![
        ScheduleBlock::Single(Arc::new(update_positions)),
        ScheduleBlock::Single(Arc::new(render_entities)),
    ]);

    runner.execute(schedule);
    loop {}
}
