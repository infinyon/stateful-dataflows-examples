import Fluvio, { Offset, Record } from '@fluvio/client';
import express, { Request, Response } from 'express';
import path from 'path';

const app = express();
app.use(express.static(path.join(__dirname, 'public')));
const port = 3000;
/* maybe make this loading a little different via a json file? */
const TOPIC_LIST = [
    {
        name: "cars",
        type: "object"
    },
    {
        name: "licenses",
        type: "array"
    },
    {
        name: "makers",
        type: "object"
    },
    {
        name: "speeding",
        type: "object"
    },
    {
        name: "saratoga",
        type: "object"
    },
    {
        name: "sunnyvale",
        type: "object"
    },
    {
        name: "violations",
        type: "object"
    },
    {
        name: "car-colors",
        type: "array"
    },
];
const PARTITION = 0;
const clients: Response[] = [];
const broadcastToClients = (message: string) => {
  clients.forEach(client => {
    client.write(`data: ${message}\n\n`);
  });
};

/*
Turns an value and obj composed of a name and type to proper json
*/
function toProperJSON(value: string, obj: {name: string, type: string}){
    if(obj.type == "object"){
        return (`${value}`).slice(0,-1)+',"type":"'+obj.name+'"}';
    } 
    else if(obj.type == "array"){
        return '{"value":'+value+',"type":"'+obj.name+'"}'
    }
    return '{"type":"error"}'
}
async function consume() {
    console.log('Connecting client to Fluvio');
    try{
        const fluvio = await new Fluvio();
        await fluvio.connect();
        for (const topic of TOPIC_LIST) {
            try {
                console.log('\tConnecting to topic: '+topic.name);
                const consumer = await fluvio.partitionConsumer(topic.name, PARTITION);
                await consumer.stream(Offset.FromBeginning(), async (record: Record) => {
                    const message = toProperJSON(record.valueString(),topic)
                    broadcastToClients(message);  
                });
            } catch (ex) {
                console.log('Error', ex);
            }
        }
    } catch (ex){
        console.log("Cannot connect to fluvio",ex)
    }
}

app.get('/', (req: Request, res: Response) => {
    res.sendFile(path.join(__dirname, 'public', 'static.html'));
});

app.get('/events', (req: Request, res: Response) => {
  res.setHeader('Content-Type', 'text/event-stream');
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Connection', 'keep-alive');

  clients.push(res);

  req.on('close', () => {
    clients.splice(clients.indexOf(res), 1);
  });
});

app.listen(port, () => {
  console.log(`Server is running on http://localhost:${port}`);
  consume();
});