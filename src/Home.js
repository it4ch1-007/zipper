//this is for the home page
import { Text } from "@mantine/core";
import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { Group, Button } from '@mantine/core';
import { NavLink } from "react-router-dom";

function Home() {

    //calls whenever the page is rendered
    
    return (
        
    //     <Group direction="vertical" align="center" style={{ marginTop: '20px' }}>
    //     {lines.map((line, index) =>
    //       <Button key={index} component={NavLink} className={classes.navLink} fullWidth>
    //         {line}
    //       </Button>
    //     )}
    //   </Group>
    <Text>Home</Text>
    );
}
export default Home;