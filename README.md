# spotify-cleanup

Destroys the leaked pipewire-pulse clients by the Spotify application on Linux.

## Why?

Spotify leaks pipewire-pulse clients like its going out of style. If unchecked, the leaked pipewire-pulse objects prevent new pipewire-pulse clients from connecting to pipewire's pulse server breaking expected behavior. This is annoying.

## How?

Run `spotify-cleanup` in a systemd user service every 30 minutes.

## Installing

### Nix Flake (recommended)

Add the following snippet to your flake inputs:

```nix
spotify-cleanup = {
    url = "github:svrana/spotify-cleanup";
    inputs.nixpkgs.follows = "nixpkgs";
}
```

#### Using the Home Manager module (recommended)

Add the following to your home-manager imports:

```nix
inputs.spotify-cleanup.homeModules.default
```

And then you may use the option to set it up, for example:

```nix
services.spotify-cleanup = {
    enable = true;
    systemdTarget = "sway-session.target";
    interval = "30m";
}
```

This method will install the program and setup a user systemd service.
