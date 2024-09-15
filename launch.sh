pwd="$PWD"
cd ~/coding/prag-portal || exit
npx tailwindcss -i ./input.css -o ./public/tailwind.css || exit
dx build || exit
cd dist || exit
RUST_BACKTRACE=1 ./prag-portal
cd "$pwd" || exit
