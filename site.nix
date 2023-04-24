{ config, lib, pkgs, ... }:
with lib;
let cfg = config.services.site;
in
{
  options.services.site = {
    enable = mkOption {
      type = types.bool;
      default = false;
      description = "Enable whydoesntmycode.work blog";
    };
  };
}
