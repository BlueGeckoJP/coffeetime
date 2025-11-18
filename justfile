db_path := env_var_or_default("DB_PATH", "sqlite://" + env_var("HOME") + "/.local/share/coffeetime/coffeetime.db?mode=rwc")
exec_path := env_var_or_default("EXEC_PATH", env_var("HOME") + "/.local/share/coffeetime/coffeetime-daemon")

build:
    cargo build --release
    cd daemon && cargo build --release
    @echo "Build completed successfully"
    @echo "The deliverables are located within target/release"

install:
    just build
    # prepare directories
    mkdir -p $HOME/.local/share/coffeetime/
    cp target/release/coffeetime-daemon $HOME/.local/share/coffeetime/
    # install systemd timers (system level)
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-startup.service | sudo tee /etc/systemd/system/coffeetime-startup.service > /dev/null
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-shutdown.service | sudo tee /etc/systemd/system/coffeetime-shutdown.service > /dev/null
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-before-sleep.service | sudo tee /etc/systemd/system/coffeetime-before-sleep.service > /dev/null
    sed -e "s|<<<EXEC_PATH>>>|{{exec_path}}|g" -e "s|<<<DB_PATH>>>|{{db_path}}|g" systemd-services/coffeetime-after-sleep.service | sudo tee /etc/systemd/system/coffeetime-after-sleep.service > /dev/null
    sudo systemctl daemon-reload
    sudo systemctl enable coffeetime-startup.service
    sudo systemctl enable coffeetime-shutdown.service
    sudo systemctl enable coffeetime-before-sleep.service
    sudo systemctl enable coffeetime-after-sleep.service
    @echo "Installation completed successfully"

uninstall:
    # remove systemd services (system level)
    sudo systemctl disable coffeetime-startup.service || true
    sudo systemctl disable coffeetime-shutdown.service || true
    sudo systemctl disable coffeetime-before-sleep.service || true
    sudo systemctl disable coffeetime-after-sleep.service || true
    sudo rm -f /etc/systemd/system/coffeetime-startup.service
    sudo rm -f /etc/systemd/system/coffeetime-shutdown.service
    sudo rm -f /etc/systemd/system/coffeetime-before-sleep.service
    sudo rm -f /etc/systemd/system/coffeetime-after-sleep.service
    sudo systemctl daemon-reload
    @echo "Uninstallation completed successfully"

uninstall-all:
    just uninstall
    rm -rf $HOME/.local/share/coffeetime/
    @echo "Complete uninstallation completed successfully"
