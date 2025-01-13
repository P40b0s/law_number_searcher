cd win  
sudo dnf config-manager setopt nasm.enabled=1

sudo dnf install -y cmake \
x86_64-w64-mingw32-g++ \
x86_64-w64-mingw32-gcc \
perl \
go \
nasm