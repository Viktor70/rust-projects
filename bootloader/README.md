# bootloader
Это простой загрузчик, написанный на языке ассемблера NASM. При загрузке выводит сообщение, ждёт нажатия любой клавиши, выключает виртуальную машину.
зачем нужен:
 - иллюстрирует, как начинается загрузка системы
 - показывает, как работает процессор без операционной системы
 - есть пример выполнения прерывания
 - есть пример работы с памятью
 - есть пример работы с перефирией (keyboard)
 - использует магические байты (0xAA55)

## Как собрать
```

nasm -f bin bootloader.asm -o bootloader.bin
```

## как запусть в qemu

в терминале
```
qemu-system-x86_64 -drive format=raw,file=bootloader.bin -nographic -serial mon:stdio
```

Или для графического режима:
```
qemu-system-x86_64 -drive format=raw,file=bootloader.bin
```

## Как запустить в qemu с записью в загрузочный сектор
```
dd if=/dev/zero of=disk.img bs=1024 count=1440
dd if=bootloader.bin of=disk.img conv=notrunc
```

запускаем в консоли
```
qemu-system-x86_64 -nographic -drive format=raw,file=disk.img
```
в графическом режиме
```
qemu-system-x86_64 -fda disk.img
```

проверка загрузочной записи
```
hexdump -C disk.img
```

## Как запустить в vmware

sudo apt install nasm
sudo dd if=bootloader.bin of=/dev/sda conv=notrunc
```

