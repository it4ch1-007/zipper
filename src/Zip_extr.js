import React, {useEffect} from 'react';
import { useLocation } from 'react-router-dom';
import { createStyles} from '@mantine/core';
import { ScrollArea } from '@mantine/core';
import { Button } from '@mantine/core';

function Zip_extr() {
    const location = useLocation();
    const {zipName} = location.state || {};
    
    useEffect(() => {
        const fetchData = async () => {
          try {
            console.log("Fetching data from the zip:", zipName);
            // Perform further actions with zipname if needed
          } catch (err) {
            console.log("Error fetching the zip name", err);
          }
        };
          fetchData();

      }, [zipName]);
    const useStyles = createStyles((theme) => ({
        scrollArea: {
          maxHeight: 50,
          maxWidth: 50,
          overflow: 'auto',
          border: `1px solid ${theme.colors.gray[4]}`, // Optional: Add a border for better visibility
        },
      }));


    return (
        <div>
        <h1>
        Zip: {zipName}
        </h1>
        <Button>Extract zip</Button>
        {/* <p>
        Zip Name: {zipname}
        </p> */}
        <ScrollArea className='classes.scrollArea'>
      {"helloooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo"}
    </ScrollArea></div>
    );
};

export default Zip_extr;