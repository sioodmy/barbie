self: {
  config, 
  lib,
  pkgs,
  inputs,
  ...
}: let

  inherit (lib) mkIf mkEnableOption mkOption maintainers;

  cfg = config.services.crabpulsar;
in {  
  meta.maintainers = with maintainers; [sioodmy];

  options.services.crabpulsar = with lib.types; {
    enable = mkEnableOption "crabpulsar";
    package = mkOption {
      type = package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
      description = "CrabPulsar package to use";
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];
     systemd.user.services.waybar = {
        Unit = {
          Description =
            "My custom status bar for Hyprland";
          Documentation = "https://github.com/sioodmy/crabpulsar";
          PartOf = [ "graphical-session.target" ];
          After = [ "graphical-session-pre.target" ];
        };

        Service = {
          ExecStart = "${cfg.package}/bin/crabpulsar";
          ExecReload = "${pkgs.coreutils}/bin/kill -SIGUSR2 $MAINPID";
          Restart = "on-failure";
          KillMode = "mixed";
        };

        Install = { WantedBy = [ "hyprland-session.target"]; };
      };
    
  };

}
