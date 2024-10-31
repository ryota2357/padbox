{
  lib,
  stdenv,
  rustPlatform,
  installShellFiles,
}:
rustPlatform.buildRustPackage {
  name = "padbox";

  src = lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ installShellFiles ];

  postInstall = lib.optionalString (stdenv.buildPlatform.canExecute stdenv.hostPlatform) ''
    installShellCompletion --cmd padbox \
      --bash <($out/bin/padbox completion bash) \
      --fish <($out/bin/padbox completion fish) \
      --zsh <($out/bin/padbox completion zsh)
  '';

  meta = with lib; {
    description = "A CLI tool to quickly set up custom local playgrounds";
    homepage = "https://github.com/ryota2357/padbox";
    license = licenses.mit;
    mainProgram = "padbox";
  };
}
