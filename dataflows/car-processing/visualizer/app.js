"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const client_1 = __importStar(require("@fluvio/client"));
const express_1 = __importDefault(require("express"));
const path_1 = __importDefault(require("path"));
const app = (0, express_1.default)();
app.use(express_1.default.static(path_1.default.join(__dirname, 'public')));
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
const clients = [];
const broadcastToClients = (message) => {
    clients.forEach(client => {
        client.write(`data: ${message}\n\n`);
    });
};
/*
Turns an value and obj composed of a name and type to proper json
*/
function toProperJSON(value, obj) {
    if (obj.type == "object") {
        return (`${value}`).slice(0, -1) + ',"type":"' + obj.name + '"}';
    }
    else if (obj.type == "array") {
        return '{"value":' + value + ',"type":"' + obj.name + '"}';
    }
    return '{"type":"error"}';
}
function consume() {
    return __awaiter(this, void 0, void 0, function* () {
        console.log('Connecting client to Fluvio');
        try {
            const fluvio = yield new client_1.default();
            yield fluvio.connect();
            for (const topic of TOPIC_LIST) {
                try {
                    console.log('\tConnecting to topic: ' + topic.name);
                    const consumer = yield fluvio.partitionConsumer(topic.name, PARTITION);
                    yield consumer.stream(client_1.Offset.FromBeginning(), (record) => __awaiter(this, void 0, void 0, function* () {
                        const message = toProperJSON(record.valueString(), topic);
                        broadcastToClients(message);
                    }));
                }
                catch (ex) {
                    console.log('Error', ex);
                }
            }
        }
        catch (ex) {
            console.log("Cannot connect to fluvio", ex);
        }
    });
}
app.get('/', (req, res) => {
    res.sendFile(path_1.default.join(__dirname, 'public', 'static.html'));
});
app.get('/events', (req, res) => {
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
