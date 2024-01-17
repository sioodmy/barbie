self: {
  config, 
  lib,
  pkgs,
  inputs,
  ...
}: let

  inherit (lib) mkIf mkEnableOption mkOption maintainers;

  cfg = config.services.barbie;
in {  
  meta.maintainers = with maintainers; [sioodmy];

  options.services.barbie = with lib.types; {
    enable = mkEnableOption "barbie";
    package = mkOption {
      type = package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      description = "Barbie package to use";
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];
     systemd.user.services.barbie= {
        Unit = {
          Description =
            "My custom status bar for Hyprland";
          Documentation = "https://github.com/sioodmy/barbie";
          PartOf = [ "graphical-session.target" ];
          After = [ "graphical-session-pre.target" ];
        };

        Service = {
          ExecStart = "${cfg.package}/bin/barbie";
          ExecReload = "${pkgs.coreutils}/bin/kill -SIGUSR2 $MAINPID";
          Restart = "on-failure";
          KillMode = "mixed";
        };

        Install = { WantedBy = [ "hyprland-session.target"]; };
      };
    
  };

}
