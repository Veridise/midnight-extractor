# Launching jobs

Once you have [uploaded a version](./uploading.md) of the extracted circuits to AuditHub you can 
start running tools with them.

We are going to see how to launch a Picus job from the CLI. 
Each individual Picus file needs to be run as a separate task. Pick one of the 
Picus files from a version uploaded to AuditHub and note its path relative to 
the directory you uploaded it from. This is the path AuditHub will use to locate the file previouly uploaded.

For example, to run a job on one of these files for up to one hour 
you can run the following command. 

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

This will wait until the job has completed in AuditHub. If you don't pass the `--wait` flag the CLI returns immediately
with the task ID. This ID can be passed to `ah monitor-task -t $task_id` for checking the status of the task.

> You can run `ah start-picus-v2-task --help` to get more details about the possible flags and arguments.

Similarly to the upload, if you configured the `AUDITHUB_PROJECT_ID` and `AUDITHUB_ORGANIZATION_ID` environment variables 
you can drop the `--project-id` and `--organization-id` flags since the CLI can read them from the environment.

In some cases Picus requires additional configuration; 
if you passed the `--prelude` flag while extracting circuits with either `spread` 
or `automaton` then you need to pass additional flags. If you are unsure wether the Picus file you want to analyze 
was generated using that flag, inspect the first few lines of the file. If the tool injected one of the preludes it 
will contain comments indicating it. 

Pass the additional flags to the CLI according to the table below.

| Prelude | Additonal Picus flags |
|---------|-----------------------|
| `spread` | `--assume-deterministic Spread,Unspread` |
| `automaton` | `--assume-deterministic Automaton` |
| None | `--empty-assume-deterministic` |
