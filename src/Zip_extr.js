import React, {useEffect, useState} from 'react';
import { useLocation } from 'react-router-dom';
import { createStyles} from '@mantine/core';
import { ScrollArea } from '@mantine/core';
import { Button } from '@mantine/core';
import {Modal,TextInput} from '@mantine/core';
import { invoke } from '@tauri-apps/api';
import {Alert} from '@mantine/core';


function Zip_extr() {

    let flag=false;//the variable to ensure that the read_zip_files function is called just once and to synchronize the updation of the prompt variable with the reading of the prompt variable
    const location = useLocation(); //capturing the props passed from Home.js
    const {zipName} = location.state || {}; //setting zipname to store the prop state 
    const [isModalOpen, setIsModalOpen] = useState(false); //Store the state of the password prompt modal
    const [inputValue, setInputValue] = useState('trllo');//Store the inputvalue of the password 
    const closeModal = () => setshowPrompt(true); //function that closes the modal 
    const [fileNames, setFileNames] = useState([]);//store the filenames inside the zip to show in the file tree.
    const [showAlert,setShowAlert] = useState(false);//show alert when the zip is extracted
    const [prompt,setshowPrompt] = useState();//To store if the zip requires a password or not
    const [responseMetadata,setresponseMetadata] = useState('');//Store the metadata of the zip file
    
    //When submit is clicked inside the password prompt modal
    const handleSubmission = (event) => {
      closeModal();
      invoke('read_zip_files_pswd', { zippath: zipName, pswd: inputValue })
      .then(response => {
        console.log("flag value: ",flag);
        setFileNames(response);
        flag=true;
      })
      .catch(error => {
        invoke('error_printer_pswd');
      });
    };

    //When the password inside the inputValue text box is changed
    const handlePasswordChange = (event) => {
          
      setInputValue(event.currentTarget.value);
    };

    //To handle the reading of zip files in the case where the zip is not encrypted.
    const handlereadzipfiles = async() => {
        
        if(!flag){
        flag=true;
        console.log("inside read zip files handler: ",prompt);
        if(prompt){
          invoke('read_zip_files', { zippath: zipName ,pswd:inputValue})
        .then(response => {
          setFileNames(response);
        })
        .catch(error => {
          invoke('error_printer');

        }
      
      ,[]);
        }
      }
      }

    //To handle the extraction of zip files in both cases encrypted and not encrypted.
    const handleExtraction = async() =>{
        if(prompt){
        await invoke('extract_zip',{zippath:zipName,pswd:inputValue});
        }
        else{
          await invoke('extract_zip_pswd',{zipPath:zipName,pswd:inputValue});
        }
        await invoke('config_write',{zipPath:zipName});
        setShowAlert(true);

        setTimeout(() => {
          setShowAlert(false);
        },3000);
                
    };
   
    //To handle the fetching of metadata and representing it as the JSON string
    const handleMetadata = async () => {
        try{
        const responseMetadata = await invoke('read_metadata',{archive: zipName});
        setresponseMetadata(responseMetadata);
        
        }
        catch(err){
            console.error("Failed to fetch metadata");
        }

    }


    //To check if the zip is encrypted or not
    const handlePriorCheck = async() => {
      
      const response = await invoke('prior_check',{zippath:zipName});
        setshowPrompt(response);
        //Sets the prompt variable to true if the zip not requires a password.
  }

      //To call some the handler functions as soon as the page loads
      useEffect(() => {
            handleMetadata();
            handlePriorCheck();
        }, [zipName]);
        useEffect(() => {
          console.log("value of prompt inside useEffect: ",prompt);
          if (prompt) {
            handlereadzipfiles();
          }
        }, [zipName,prompt]);    


    const useStyles = createStyles((theme) => ({
      scrollArea: {
        overflow: 'auto',
        border: `1px solid ${theme.colors.dark[7]}`, // Optional: Add a border for better visibility
        backgroundColor: theme.colors.dark[5],
      },
    }));
    const {classes} = useStyles();


    return (
        
        <div>
            <h1>
                Zip:</h1> {zipName}<br></br>
            
            <Button onClick={handleExtraction}>Extract zip</Button>
            {showAlert && (
              <div style={{ position: 'fixed', bottom: '20px', right: '20px', zIndex: 1000 }}>
        <Alert variant="light" color="blue" title="Alert title">
          Zip {zipName} extracted in the same directory...
        </Alert>
        </div>
      )}
      <p>Zip metadata: </p>
            <ScrollArea w={300} h={200} scrollbars="x" type='hover' className={classes.scrollArea}>
        {responseMetadata}
    </ScrollArea>       
    <br></br>
    <br></br>
      <p>Zip Files tree: </p>
      {/* mapping the filenames inside the zip acdordingy with their path names */}
    <ScrollArea style={{ height: 300, width: 700 }} className={classes.scrollArea}>
      {fileNames.map((fileName, index) => (
        <div key={index}>{fileName}</div>
      ))}
    </ScrollArea>  
    {/* Showing the prompt password modal only if the password is required by the zip */}
    <Modal opened={!prompt} onClose={closeModal} title="Password Test">
        <TextInput
          label="Password"
          placeholder="Enter password"
          onChange={handlePasswordChange}
        />
        <Button onClick={handleSubmission}>Submit</Button>
      </Modal>   
    </div>
    );
    
}

export default Zip_extr;