# Uploading new versions

Make sure you went through the steps in [Creating a new project](./first-project.md) before continuing.

After extracting a new version of the circuits but before running Picus with them, they need to be uploaded 
to AuditHub. This can be accomplised with either the UI or with the CLI. In this document we are going to see 
how to do it with the CLI, that you should have [configured already](./intro.md).

Each version needs to have a name. As an example we are going to use the date of submission and the 
sha256 of the generated picus files.
Run the following commands, assuming the picus files are in the default output folder (`picus_files`).


```bash
date=$(date "+%Y-%m-%d-%H-%M")
sum=$(find picus_files -type f | sort | xargs cat | sha256sum | head -c 8)
version_id=$(ah create-version-via-local-archive \
    --name "Picus-files-$date-$sum" \
    --source-folder picus_files \
    --project-id $PROJECT_ID \
    --organization-id $ORGANIZATION_ID)
```

If you configured the `AUDITHUB_PROJECT_ID` and `AUDITHUB_ORGANIZATION_ID` environment variables 
you can drop the `--project-id` and `--organization-id` flags since the CLI can read them from the environment.
Note that we stored the output of the CLI in a variable named `$version_id`. We need it for identifying the 
version when [launching jobs](./jobs.md).
