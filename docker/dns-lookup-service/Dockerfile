FROM rust:1.23.0
ADD config.toml config.toml
ADD log_config.yaml log_config.yaml
ADD rust_cmd.sh rust_cmd.sh
ADD rust-bin rust-bin
RUN chmod +x rust-bin rust_cmd.sh

CMD [ "sh", "rust_cmd.sh" ]

EXPOSE 21021/tcp
