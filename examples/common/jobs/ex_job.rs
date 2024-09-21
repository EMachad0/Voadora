use uuid::Uuid;
use voadora::Job;

pub(crate) const JOB_ID: Uuid = Uuid::from_u128(0x5906dcc1e014fa58787c9fd58a0064c);

inventory::submit! {
    Job {
        uuid: JOB_ID,
        perform: |params| {
            println!("Performing TestJob with params: {:?}", params);
        },
    }
}
