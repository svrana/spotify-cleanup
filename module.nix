{ config, lib, pkgs, ... }:
with lib;
let
  cfg = config.services.spotify-cleanup;
in
{
  options.services.spotify-cleanup = {
    enable = mkEnableOption "spotify-cleanup";

    package = mkOption {
      type = types.package;
      default = pkgs.callPackage ./default.nix { };
      description = ''
        The spotify-cleanup package to use.
      '';
    };

    systemdTarget = mkOption {
      type = types.str;
      default = "graphical-session.target";
      example = "sway-session.target";
      description = ''
        systemd target to bind to.
      '';
    };

    interval = mkOption {
      type = types.str;
      default = "30m";
      description = ''
        How often spotify-cleanup should run.
      '';
    };
  };

  config = mkIf cfg.enable {
    systemd.user.services.spotify-cleanup = {
      Unit = {
        Description = "Cleanup Spotify's leaked pulse objects";
        Documentation = "https://github.com/svrana/spotify-cleanup";
      };

      Install.WantedBy = [ cfg.systemdTarget ];

      Service = {
        Type = "oneshot";
        ExecStart = "${lib.getExe spotify-cleanup}";
        Restart = "always";
        RestartSec = 10;
      };
    };

    systemd.user.timers.spotify-cleanup = {
      Unit.Description = "Cleanup Spotify's leaked pulse objects";
      Timer = {
        Unit = "spotify-cleanup";
        OnBootSec = cfg.interval;
        OnUnitActiveSec = cfg.interval;
      };
      Install.WantedBy = [ "timers.target" ];
    };
  };
}
