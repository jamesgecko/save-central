package main

import (
    "os"
    "errors"
    "syscall"
    "encoding/csv"
    // "golang.org/x/sys"
    "github.com/nyaosorg/go-windows-junction"
)

func main1() error {
    if len(os.Args) < 3 {
        return errors.New("go run example.go TARGET MOUNTPT")
    }
    if err := junction.Create(os.Args[1], os.Args[2]); err != nil {
        return err
    }
    result, err := os.Readlink(os.Args[2])
    if err != nil {
        return err
    }
    println(os.Args[2], " is linked to ", result)
    return nil
}

func main() {
    if err := main1(); err != nil {
        println(err.Error())
        os.Exit(1)
    }
}

// func isJunction(path string) bool { }
// func moveAndJunction(oldPath string, newPath string) {}
// func restoreJunction(normalPath string, savePath string) {}
// func createJunction(target string, junction string) {}
// funcion moveSave(oldPath string, newPath string) {}
// func linkSave(target string, junction string) {}

func hideDirectory(path string) {
    p, err := syscall.UTF16PtrFromString(path)
    if err != nil {
        panic(err)
    }

    attrs, err := syscall.GetFileAttributes(p)
    if err != nil {
        panic(err)
    }

    attrs |= syscall.FILE_ATTRIBUTE_HIDDEN
    syscall.SetFileAttributes(p, attrs)
}
