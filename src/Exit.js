import React, { useEffect } from "react";
import {Text} from "@mantine/core";
import { appWindow } from "@tauri-apps/api/window";

export default function Exit(){
    const handleClose = async() => {
        await appWindow.close();
    }

    useEffect(() => {
        handleClose();//Exiting the main application window
    })
    return(
        <></>
    )
}