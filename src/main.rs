enum Step {
	Download(DownloadStep),
	ExecuteDownload(ExecuteDownloadStep),
	CreateDir(CreateDirStep),
	RemoveDir(RemoveDirStep),
	Untar(UntarStep),
	PathUpdate(PathUpdateStep),
}

type RunAfter = Vec<String>;

struct DownloadStep {
	run_after: RunAfter,
	url: String,
	output: String,
}

struct ExecuteDownloadStep {
	run_after: RunAfter,
	artifact: String,
	args: Vec<String>,
}

struct CreateDirStep {
	run_after: RunAfter,
	path: String,
}

type RemoveDirStep = CreateDirStep;

struct UntarStep {
	run_after: RunAfter,
	artifact: String,
	args: Vec<String>,
}

struct PathUpdateStep {
	run_after: RunAfter,
}

struct VerificationCommand {
	command: String,
	args: Vec<String>,
	expected: String,
}

struct Application {
	name: String,
	run_after: Vec<String>,
	verification: VerificationCommand,
	steps: Vec<Step>,
}

struct Profile {
	name: String,
	applications: Vec<Application>,
}

struct Profiles {
	profiles: Vec<Profile>,
}

fn main() {
	println!("Hello, world!");
}
