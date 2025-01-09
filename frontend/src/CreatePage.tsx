import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';

const CreatePage: React.FC = () => {
    const [profilePicture, setProfilePicture] = useState<File | null>(null);
    const [name, setName] = useState('');
    const [ticker, setTicker] = useState('');
    const [biography, setBiography] = useState('');
    const [agentType, setAgentType] = useState('');
    const navigate = useNavigate();

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        const formData = new FormData();
        if (profilePicture) {
            formData.append('profilePicture', profilePicture);
        }
        formData.append('name', name);
        formData.append('ticker', ticker);
        formData.append('biography', biography);
        formData.append('agentType', agentType);

        try {
            const response = await fetch('http://localhost:3030/createAgent', {
                method: 'POST',
                body: formData,
            });

            if (!response.ok) {
                throw new Error(`HTTP error! status: ${response.status}`);
            }

            const data = await response.json();
            console.log('Agent created successfully:', data);

            navigate('/');
        } catch (error) {
            console.error('Error creating agent:', error);
        }
    };

    return (
        <div className="bg-white shadow-md rounded-lg p-6 w-full max-w-4xl">
            <h1 className="text-2xl font-bold mb-4">Create AI Agent</h1>
            <form onSubmit={handleSubmit} className="flex flex-col space-y-4">
                <div>
                    <label className="block">Profile Picture:</label>
                    <input
                        type="file"
                        accept="image/*"
                        onChange={e => setProfilePicture(e.target.files?.[0] || null)}
                        required
                    />
                </div>
                <div>
                    <label className="block">AI Agent Name:</label>
                    <input
                        type="text"
                        value={name}
                        onChange={e => setName(e.target.value)}
                        className="border p-2 w-full"
                        required
                    />
                </div>
                <div>
                    <label className="block">Ticker:</label>
                    <input
                        type="text"
                        value={ticker}
                        onChange={e => setTicker(e.target.value)}
                        className="border p-2 w-full"
                        required
                    />
                </div>
                <div>
                    <label className="block">AI Agent Biography:</label>
                    <textarea
                        value={biography}
                        onChange={e => setBiography(e.target.value)}
                        className="border p-2 w-full"
                        required
                    />
                </div>
                <div>
                    <label className="block">Agent Type:</label>
                    <select
                        value={agentType}
                        onChange={e => setAgentType(e.target.value)}
                        className="border p-2 w-full"
                        required
                    >
                        <option value="">Select Type</option>
                        <option value="Type1">Type1</option>
                        <option value="Type2">Type2</option>
                    </select>
                </div>
                <button
                    type="submit"
                    className="bg-green-500 text-white px-4 py-2 rounded"
                >
                    Create
                </button>
            </form>
        </div>
    );
};

export default CreatePage;