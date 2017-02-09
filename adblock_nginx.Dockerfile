FROM nginx:stable

COPY files/nginx.conf /etc/nginx/nginx.conf

EXPOSE 80 443

