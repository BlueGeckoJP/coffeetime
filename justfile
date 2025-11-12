db_path := env_var_or_default("DB_PATH", "sqlite://" + env_var("HOME") + "/.local/share/coffeetime/coffeetime.db?mode=rwc")
exec_path := env_var_or_default("EXEC_PATH", env_var("HOME") + "/.local/share/coffeetime/coffeetime-daemon")

build:
    cargo build --release
    cd daemon && cargo build --release
    @echo "Build completed successfully"
    @echo "The deliverables are located within target/release"

install:
    # prepare directories
    mkdir -p $HOME/.local/share/coffeetime/
    cp target/release/coffeetime-daemon $HOME/.local/share/coffeetime/
    mkdir -p $HOME/.config/systemd/user/
    # install systemd services (user level)
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-startup.service | tee $HOME/.config/systemd/user/coffeetime-startup.service > /dev/null
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-shutdown.service | tee $HOME/.config/systemd/user/coffeetime-shutdown.service > /dev/null
    systemctl --user daemon-reload
    systemctl --user enable coffeetime-startup.service
    systemctl --user enable coffeetime-shutdown.service
    # install systemd timers (system level)
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-before-sleep.service | sudo tee /etc/systemd/system/coffeetime-before-sleep.service > /dev/null
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-after-sleep.service | sudo tee /etc/systemd/system/coffeetime-after-sleep.service > /dev/null
    sudo systemctl daemon-reload
    sudo systemctl enable coffeetime-before-sleep.service
    sudo systemctl enable coffeetime-after-sleep.service
    @echo "Installation completed successfully"
