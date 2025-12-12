# Launching jobs

Once you have [uploaded a version](./uploading.md) of the extracted circuit to AuditHub you can 
start running tools with them.

We are going to see how to launch a Picus job from the CLI. 
Each individual Picus file needs to be run as a separate task. Pick one of the 
Picus files in the version you last uploaded and note its path relative to 
the directory you uploaded it from. For example, to run a job for up to one hour 
run the following in the shell. 

```bash
# Example path that could be extracted 
PICUS_FILE=arithmetic/add/native/native/output.picus
ah start-picus-v2-task \
    --source $PICUS_FILE \
    --version-id $version_id \
    --project-id $PROJECT_ID \
    --organization-id $ORGANIZATION_ID \
    --time-limit 3600000 \
    --empty-assume-deterministic \
    --wait
```

This will wait until the job has completed in AuditHub. If the `--wait` flag is dropped the CLI returns immediately, 
responding with the task ID. This ID can be passed to `ah monitor-task -t $task_id` for checking the status of the task.

> You can run `ah start-picus-v2-task --help` to get more details about the possible flags and arguments.

Similarly to the upload, if you configured the `AUDITHUB_PROJECT_ID` and `AUDITHUB_ORGANIZATION_ID` environment variables 
you can drop the `--project-id` and `--organization-id` flags since the CLI can read them from the environment.

Picus requires a bit of configuration 
in some cases; if you passed the `--prelude` flag while extracting circuits with either `spread` 
or `automaton` then you need to pass additional flags.

| Prelude | Additonal Picus flags |
|---------|-----------------------|
| `spread` | `--assume-deterministic Spread,Unspread` |
| `automaton` | `--assume-deterministic Automaton` |
| None | `--empty-assume-deterministic` |
