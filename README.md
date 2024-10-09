# spotify-cleanup

Destroys the leaked pipewire-pulse clients by the Spotify application on Linux.

## Why?

Spotify leaks its pulse clients. This seems to happen when its syncing state with another
version of spotify running on your phone, car, etc. If unchecked, these leaked clients
will prevent new clients from connecting and cause problems like your audio not working,
volume control broken, etc. While I believe this to be a problem for both pulseaudio and
pipewire-pulse audio systems, this solution works only for pipewire as that's what I use
and was more interested in learning a bit about how to interact with it than I was about
pulseaudio. If you're not using pulseaudio, you may check out this script posted [here](https://community.spotify.com/t5/Desktop-Linux/Spotify-opens-too-many-connections-to-PulseAudio-Linux/m-p/5369848#M21001-)
in a user comment.


## How?

Run `spotify-cleanup` in a systemd user service every 30 minutes.

## Installing

### Nix Flake (recommended)

Add the following snippet to your flake inputs:

```nix
spotify-cleanup = {
    url = "github:svrana/spotify-cleanup";
    inputs.nixpkgs.follows = "nixpkgs";
};
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
    systemdTarget = "graphical-session.target";
    interval = "30m";
};
```

This method will install the program and setup a user systemd service.
