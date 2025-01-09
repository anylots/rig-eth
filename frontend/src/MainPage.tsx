import React, { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';

type Agent = {
    id: string;
    name: string;
    marketCap: number;
    TVL: number;
    topTweet: string;
};

const MainPage: React.FC = () => {
    const [agents, setAgents] = useState<Agent[]>([]);
    const [page, setPage] = useState(1);
    const navigate = useNavigate();

    const tempData: Agent[] = [
        {
            id: '1',
            name: 'Agent Alpha',
            marketCap: 500000,
            TVL: 200000,
            topTweet: 'This is the top tweet for Agent Alpha.',
        },
        {
            id: '2',
            name: 'Agent Beta',
            marketCap: 300000,
            TVL: 150000,
            topTweet: 'This is the top tweet for Agent Beta.',
        },
    ];

    const fetchAgents = async () => {
        try {
            // const response = await fetch(`http://localhost:3030/queryAgents?page=${page}&limit=100`);
            // if (!response.ok) {
            //     throw new Error(`HTTP error! status: ${response.status}`);
            // }
            // const data = await response.json();
            // setAgents(data);
            setAgents(tempData);

        } catch (error) {
            console.error(error);
        }
    };

    useEffect(() => {
        fetchAgents();
    }, [page]);

    return (
        <div className="bg-white shadow-md rounded-lg p-6 w-full max-w-7xl">
            <div className="flex justify-between items-center mb-4">
                <h1 className="text-2xl font-bold">AI Agents</h1>
                <button
                    className="bg-blue-500 text-white px-4 py-2 rounded"
                    onClick={() => navigate('/create')}
                >
                    Create
                </button>
            </div>
            <table className="w-full bg-white border border-gray-200">
                <thead>
                    <tr className="text-left">
                        <th className="py-4 border-b">AI Agents</th>
                        <th className="py-4 border-b">Market Cap</th>
                        <th className="py-4 border-b">TVL</th>
                        <th className="py-4 border-b">Top Tweet</th>
                    </tr>
                </thead>
                <tbody>
                    {agents.map(agent => (
                        <tr key={agent.id}>
                            <td className="py-4 border-b flex items-center">
                                {/* add image */}
                                <img
                                    src="/image_default.png"
                                    alt="Agent"
                                    className="w-12 h-12 mr-2 rounded-full"
                                />
                                {agent.name}
                            </td>
                            <td className="py-4 border-b">{agent.marketCap}</td>
                            <td className="py-4 border-b">{agent.TVL}</td>
                            <td className="py-4 border-b">{agent.topTweet}</td>
                        </tr>
                    ))}
                </tbody>
            </table>
            <div className="flex justify-center mt-4">
                <button
                    className="bg-gray-300 px-4 py-2 mr-2 rounded"
                    onClick={() => setPage(prev => Math.max(prev - 1, 1))}
                >
                    Previous
                </button>
                <button
                    className="bg-gray-300 px-4 py-2 ml-2 rounded"
                    onClick={() => setPage(prev => prev + 1)}
                >
                    Next
                </button>
            </div>
        </div>
    );
};

export default MainPage;