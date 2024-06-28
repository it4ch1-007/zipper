import React from "react";
import {Text,Anchor,Image} from "@mantine/core";


export default function About(){
    return(
        <>
        <h3>About the developer</h3>
        <Image
      radius="md"
      src="https://avatars.githubusercontent.com/u/133276365?s=400&u=0ce10a008c4b40bcb84853fc079715d152a59c36&v=4" width={200} height={200}
    />
        <Text> Hey I am Akshit Singh (aka it4ch1), an undergraduate from IIT Roorkee and a core member of InfoSecIITR. <br></br>
        However, I am exploring cybersecurity since my first year of college in the domain of reverse engineering <br></br> and other low-level stuff like malwares that made me fall in love with Rust and CPP
        I love writing Rust and C/C++ <br></br>and also Reverse engineering the softwares written in them when required.<br></br>
        <br></br>
        </Text>
        <h3>About the project</h3>
        <Text>
        Among my projects, <b>Zipper</b> stands out as a significant achievement, developed entirely using Rust.<br></br>
        It utilises <b>Tauri</b>, a Rust framework just like electronJS that helps to build desktop web applications.<br></br>
        Apart from the normal zip extraction functionality, Zipper provides us the recent files that we <br></br> have extracted using it.
        Also it has a <b>try-catch based password prompt functionality</b> that reduces user's time and increases efficiency.<br></br>
        Not only extraction but it also gives us the description of files and the metadata of the zip without even opening that zip<br></br>
        giving a very descriptive idea of the contents of the zip file.<br></br>
        </Text>
        <br></br><br></br><br></br>
        <Text>
            This project is open-source:<br></br> Checkout the source code:</Text>
            <Anchor href= "https://github.com/it4ch1-007/zipper">github</Anchor>
            <br></br>
            Also Checkout my blogs: <br></br>
            <Anchor href= "https://it4ch1-007.github.io/">blogs</Anchor>
            
        </>
        
    )
}