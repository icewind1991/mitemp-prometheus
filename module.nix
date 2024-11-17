{ config
, lib
, pkgs
, ...
}:
with lib; let
  cfg = config.services.mitemp;
  format = pkgs.formats.toml { };
  configFile = format.generate "mitemp-config.toml" {
    inherit (cfg) names;
    listen = {
      inherit (cfg) socket;
    };
  };
in
{
  options.services.mitemp = {
    enable = mkEnableOption "mitemp";

    names = mkOption {
      type = types.attrs;
      default = { };
      description = "Names for mitemp sensors";
    };

    socket = mkOption {
      type = types.str;
      default = "/run/mitemp/mitemp.sock";
      description = "socket to listen on";
    };

    package = mkOption {
      type = types.package;
      defaultText = literalExpression "pkgs.mitemp-prometheus";
      description = "package to use";
    };
  };

  config = mkIf cfg.enable {
    users.users.mitemp = {
      isSystemUser = true;
      group = "mitemp";
    };
    users.groups.mitemp = {};

    services.dbus.packages = [cfg.package];
    systemd.services."mitemp" = {
      wantedBy = [ "multi-user.target" ];
      after = [ "dbus.service" ];

      serviceConfig = {
        ExecStart = "${cfg.package}/bin/mitemp-prometheus ${configFile}";

        Restart = "on-failure";
        User = "mitemp";
        PrivateTmp = true;
        ProtectSystem = "strict";
        ProtectHome = true;
        NoNewPrivileges = true;
        ProtectClock = true;
        CapabilityBoundingSet = true;
        ProtectKernelLogs = true;
        ProtectControlGroups = true;
        SystemCallArchitectures = "native";
        ProtectKernelModules = true;
        RestrictNamespaces = true;
        MemoryDenyWriteExecute = true;
        ProtectHostname = true;
        LockPersonality = true;
        ProtectKernelTunables = true;
        RestrictAddressFamilies = [ "AF_UNIX" ];
        RuntimeDirectory = "mitemp";
        RestrictRealtime = true;
        ProtectProc = "invisible";
        SystemCallFilter = [ "@system-service" "~@resources" "~@privileged" ];
        IPAddressDeny = "any";
        PrivateUsers = true;
        ProcSubset = "pid";
        RemoveIPC = true;
        PrivateDevices = true;
        RestrictSUIDSGID = true;
        BindPaths = [ "/run/dbus" ];
      };

      confinement = {
        enable = true;
        binSh = null;
      };
    };
  };
}
