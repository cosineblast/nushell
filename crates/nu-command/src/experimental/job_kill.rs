use nu_engine::command_prelude::*;
use nu_protocol::engine::JobId;

#[derive(Clone)]
pub struct JobKill;

impl Command for JobKill {
    fn name(&self) -> &str {
        "job kill"
    }

    fn description(&self) -> &str {
        "Kill a background job."
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("job kill")
            .category(Category::Experimental)
            .required("id", SyntaxShape::Int, "The id of the job to kill.")
            .input_output_types(vec![(Type::Nothing, Type::Nothing)])
            .allow_variants_without_examples(true)
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["halt", "stop", "end", "close"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let head = call.head;

        let id: i64 = call.req(engine_state, stack, 0)?;

        let id: JobId = id as JobId;

        let mut jobs = engine_state.jobs.lock().expect("jobs lock is poisoned!");

        if jobs.lookup(id).is_none() {
            return Err(ShellError::JobNotFound { id, span: head });
        }

        jobs.kill_and_remove(id).map_err(|err| {
            ShellError::Io(IoError::new_internal(
                err.kind(),
                "Failed to kill the requested job",
                nu_protocol::location!(),
            ))
        })?;

        Ok(Value::nothing(head).into_pipeline_data())
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "let id = job spawn { sleep 10sec }; job kill $id",
            description: "Kill a newly spawned job",
            result: None,
        }]
    }
}
