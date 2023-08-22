FROM openresty/openresty

WORKDIR /app

COPY app/nginx.template.conf nginx.template.conf
COPY app/replace.sh replace.sh
COPY app/entrypoint.sh entrypoint.sh

ENTRYPOINT /app/entrypoint.sh
