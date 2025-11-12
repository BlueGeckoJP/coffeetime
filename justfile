build:
    cargo build --release
    cd daemon && cargo build --release
    @echo "Build completed successfully"
    @echo "The deliverables are located within target/release"

install:
    mkdir -p $HOME/.local/share/coffeetime/
    cp target/release/coffeetime-daemon $HOME/.local/share/coffeetime/
    mkdir -p $HOME/.config/systemd/user/
    cp systemd-services/* $HOME/.config/systemd/user/
    systemctl --user daemon-reload
    systemctl --user enable coffeetime-startup.service
    systemctl --user enable coffeetime-shutdown.service
    systemctl --user enable coffeetime-before-sleep.service
    systemctl --user enable coffeetime-after-sleep.service
    @echo "Installation completed successfully"
