use codexist_arg0::arg0_dispatch_or_else;
use codexist_common::CliConfigOverrides;
use codexist_mcp_server::run_main;

fn main() -> anyhow::Result<()> {
    arg0_dispatch_or_else(|codexist_linux_sandbox_exe| async move {
        run_main(codexist_linux_sandbox_exe, CliConfigOverrides::default()).await?;
        Ok(())
    })
}
