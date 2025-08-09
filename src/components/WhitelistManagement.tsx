import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { 
  Settings, 
  Plus, 
  Trash2, 
  Shield, 
  Users,
  CheckCircle,
  AlertCircle,
  Info,
  Search,
  Copy,
  ExternalLink
} from 'lucide-react';

interface WhitelistEntry {
  id: string;
  address: string;
  name: string;
  addedAt: string;
  status: 'active' | 'pending' | 'expired';
}

const WhitelistManagement: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'add' | 'manage'>('add');
  const [isAdding, setIsAdding] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);
  const [searchTerm, setSearchTerm] = useState('');

  const [formData, setFormData] = useState({
    address: '',
    name: '',
    description: '',
  });

  // Mock data for whitelist entries
  const whitelistEntries: WhitelistEntry[] = [
    {
      id: '1',
      address: '9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM',
      name: 'Treasury Wallet',
      addedAt: '2024-01-15',
      status: 'active'
    },
    {
      id: '2',
      address: '7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU',
      name: 'Team Wallet',
      addedAt: '2024-01-14',
      status: 'active'
    },
    {
      id: '3',
      address: '5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1',
      name: 'Marketing Wallet',
      addedAt: '2024-01-13',
      status: 'pending'
    },
    {
      id: '4',
      address: '3xNweLHLqrxmofjLmMcL5HjHq6KXf1J8J5J5J5J5J5J5',
      name: 'Old Partner',
      addedAt: '2024-01-10',
      status: 'expired'
    },
  ];

  const filteredEntries = whitelistEntries.filter(entry =>
    entry.address.toLowerCase().includes(searchTerm.toLowerCase()) ||
    entry.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  };

  const handleAddToWhitelist = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsAdding(true);
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    setIsAdding(false);
    setShowSuccess(true);
    
    // Reset form after 3 seconds
    setTimeout(() => {
      setShowSuccess(false);
      setFormData({
        address: '',
        name: '',
        description: '',
      });
    }, 3000);
  };

  const handleRemoveFromWhitelist = async (id: string) => {
    // Simulate removal
    console.log('Removing from whitelist:', id);
  };

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active':
        return 'text-green-400 bg-green-400/20';
      case 'pending':
        return 'text-yellow-400 bg-yellow-400/20';
      case 'expired':
        return 'text-red-400 bg-red-400/20';
      default:
        return 'text-white/60 bg-white/10';
    }
  };

  return (
    <div className="space-y-8">
      {/* Header */}
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="text-center"
      >
        <div className="flex items-center justify-center space-x-3 mb-4">
          <div className="w-12 h-12 bg-gradient-to-r from-orange-500 to-red-600 rounded-xl flex items-center justify-center">
            <Settings className="w-6 h-6 text-white" />
          </div>
        </div>
        <h1 className="text-3xl font-bold gradient-text mb-2">Whitelist Management</h1>
        <p className="text-white/70 max-w-2xl mx-auto">
          Manage transfer hook whitelists to control who can transfer your Token-2022 tokens. 
          Only whitelisted addresses can perform transfers.
        </p>
      </motion.div>

      {/* Success Message */}
      {showSuccess && (
        <motion.div
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          className="card border-green-500/20 bg-green-500/10"
        >
          <div className="flex items-center space-x-3">
            <CheckCircle className="w-6 h-6 text-green-400" />
            <div>
              <h3 className="font-semibold text-green-400">Address Added to Whitelist!</h3>
              <p className="text-green-300 text-sm">The address can now transfer tokens with transfer hook validation.</p>
            </div>
          </div>
        </motion.div>
      )}

      {/* Tab Navigation */}
      <div className="flex space-x-1 bg-white/10 rounded-lg p-1">
        <button
          onClick={() => setActiveTab('add')}
          className={`flex-1 py-2 px-4 rounded-md transition-all duration-200 ${
            activeTab === 'add'
              ? 'bg-white/20 text-white'
              : 'text-white/70 hover:text-white hover:bg-white/10'
          }`}
        >
          Add Address
        </button>
        <button
          onClick={() => setActiveTab('manage')}
          className={`flex-1 py-2 px-4 rounded-md transition-all duration-200 ${
            activeTab === 'manage'
              ? 'bg-white/20 text-white'
              : 'text-white/70 hover:text-white hover:bg-white/10'
          }`}
        >
          Manage Whitelist
        </button>
      </div>

      {activeTab === 'add' ? (
        <motion.div
          key="add"
          initial={{ opacity: 0, x: -20 }}
          animate={{ opacity: 1, x: 0 }}
          className="grid lg:grid-cols-2 gap-8"
        >
          {/* Add Address Form */}
          <div className="card">
            <h2 className="text-xl font-semibold mb-6 flex items-center space-x-2">
              <Plus className="w-5 h-5" />
              <span>Add to Whitelist</span>
            </h2>

            <form onSubmit={handleAddToWhitelist} className="space-y-6">
              {/* Wallet Address */}
              <div>
                <label className="block text-sm font-medium text-white/80 mb-2">
                  Wallet Address
                </label>
                <input
                  type="text"
                  name="address"
                  value={formData.address}
                  onChange={handleInputChange}
                  placeholder="Enter Solana wallet address"
                  className="input-field w-full"
                  required
                />
              </div>

              {/* Name */}
              <div>
                <label className="block text-sm font-medium text-white/80 mb-2">
                  Name/Description
                </label>
                <input
                  type="text"
                  name="name"
                  value={formData.name}
                  onChange={handleInputChange}
                  placeholder="e.g., Treasury Wallet, Team Member"
                  className="input-field w-full"
                  required
                />
              </div>

              {/* Description */}
              <div>
                <label className="block text-sm font-medium text-white/80 mb-2">
                  Additional Notes
                </label>
                <textarea
                  name="description"
                  value={formData.description}
                  onChange={handleInputChange}
                  placeholder="Optional notes about this address..."
                  rows={3}
                  className="input-field w-full resize-none"
                />
              </div>

              {/* Submit Button */}
              <motion.button
                type="submit"
                disabled={isAdding}
                whileHover={{ scale: 1.02 }}
                whileTap={{ scale: 0.98 }}
                className="button-primary w-full flex items-center justify-center space-x-2 disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isAdding ? (
                  <>
                    <div className="w-5 h-5 border-2 border-white/30 border-t-white rounded-full animate-spin"></div>
                    <span>Adding to Whitelist...</span>
                  </>
                ) : (
                  <>
                    <Plus className="w-5 h-5" />
                    <span>Add to Whitelist</span>
                  </>
                )}
              </motion.button>
            </form>
          </div>

          {/* Info Panel */}
          <div className="space-y-6">
            {/* Whitelist Info */}
            <div className="card">
              <h3 className="text-lg font-semibold mb-4 flex items-center space-x-2">
                <Shield className="w-5 h-5" />
                <span>How Whitelists Work</span>
              </h3>
              <div className="space-y-3">
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-400 mt-0.5" />
                  <div>
                    <h4 className="font-medium">Transfer Validation</h4>
                    <p className="text-sm text-white/60">Only whitelisted addresses can transfer tokens</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-400 mt-0.5" />
                  <div>
                    <h4 className="font-medium">Automatic Checks</h4>
                    <p className="text-sm text-white/60">Transfer hooks validate every transaction</p>
                  </div>
                </div>
                <div className="flex items-start space-x-3">
                  <CheckCircle className="w-5 h-5 text-green-400 mt-0.5" />
                  <div>
                    <h4 className="font-medium">Secure Trading</h4>
                    <p className="text-sm text-white/60">AMM swaps respect whitelist restrictions</p>
                  </div>
                </div>
              </div>
            </div>

            {/* Info Card */}
            <div className="card border-blue-500/20 bg-blue-500/10">
              <div className="flex items-start space-x-3">
                <Info className="w-5 h-5 text-blue-400 mt-0.5" />
                <div>
                  <h4 className="font-medium text-blue-400">Transfer Hook Integration</h4>
                  <p className="text-sm text-blue-300 mt-1">
                    When someone tries to transfer your token, the transfer hook program checks if the sender is whitelisted. 
                    If not, the transfer fails.
                  </p>
                </div>
              </div>
            </div>

            {/* Warning Card */}
            <div className="card border-yellow-500/20 bg-yellow-500/10">
              <div className="flex items-start space-x-3">
                <AlertCircle className="w-5 h-5 text-yellow-400 mt-0.5" />
                <div>
                  <h4 className="font-medium text-yellow-400">Important</h4>
                  <p className="text-sm text-yellow-300 mt-1">
                    Adding addresses to the whitelist gives them permission to transfer your tokens. 
                    Make sure you trust the addresses you add.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </motion.div>
      ) : (
        <motion.div
          key="manage"
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          className="space-y-6"
        >
          {/* Stats */}
          <div className="grid md:grid-cols-3 gap-6">
            <div className="card text-center">
              <div className="flex items-center justify-center w-12 h-12 bg-blue-500/20 rounded-lg mx-auto mb-3">
                <Users className="w-6 h-6 text-blue-400" />
              </div>
              <h3 className="text-2xl font-bold text-white mb-1">{whitelistEntries.length}</h3>
              <p className="text-white/60">Total Addresses</p>
            </div>
            <div className="card text-center">
              <div className="flex items-center justify-center w-12 h-12 bg-green-500/20 rounded-lg mx-auto mb-3">
                <CheckCircle className="w-6 h-6 text-green-400" />
              </div>
              <h3 className="text-2xl font-bold text-white mb-1">
                {whitelistEntries.filter(e => e.status === 'active').length}
              </h3>
              <p className="text-white/60">Active Addresses</p>
            </div>
            <div className="card text-center">
              <div className="flex items-center justify-center w-12 h-12 bg-yellow-500/20 rounded-lg mx-auto mb-3">
                <AlertCircle className="w-6 h-6 text-yellow-400" />
              </div>
              <h3 className="text-2xl font-bold text-white mb-1">
                {whitelistEntries.filter(e => e.status === 'pending').length}
              </h3>
              <p className="text-white/60">Pending Addresses</p>
            </div>
          </div>

          {/* Search */}
          <div className="card">
            <div className="flex items-center space-x-3 mb-4">
              <Search className="w-5 h-5 text-white/60" />
              <input
                type="text"
                placeholder="Search addresses or names..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="input-field flex-1"
              />
            </div>
          </div>

          {/* Whitelist Table */}
          <div className="card">
            <h2 className="text-xl font-semibold mb-6 flex items-center space-x-2">
              <Users className="w-5 h-5" />
              <span>Whitelisted Addresses</span>
            </h2>
            
            <div className="space-y-4">
              {filteredEntries.map((entry) => (
                <motion.div
                  key={entry.id}
                  whileHover={{ scale: 1.02 }}
                  className="glass-effect rounded-lg p-4 border border-white/10"
                >
                  <div className="flex items-center justify-between">
                    <div className="flex items-center space-x-4">
                      <div className="flex items-center space-x-3">
                        <div className="w-10 h-10 bg-gradient-to-r from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
                          <Shield className="w-5 h-5 text-white" />
                        </div>
                        <div>
                          <h3 className="font-semibold">{entry.name}</h3>
                          <div className="flex items-center space-x-2">
                            <p className="text-sm text-white/60 font-mono">
                              {entry.address.slice(0, 8)}...{entry.address.slice(-8)}
                            </p>
                            <button
                              onClick={() => copyToClipboard(entry.address)}
                              className="text-white/40 hover:text-white/60 transition-colors"
                            >
                              <Copy className="w-4 h-4" />
                            </button>
                            <button className="text-white/40 hover:text-white/60 transition-colors">
                              <ExternalLink className="w-4 h-4" />
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                    
                    <div className="flex items-center space-x-4">
                      <div className="text-right">
                        <p className="text-sm text-white/60">Added</p>
                        <p className="font-semibold">{entry.addedAt}</p>
                      </div>
                      <div className="flex items-center space-x-2">
                        <span className={`px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(entry.status)}`}>
                          {entry.status}
                        </span>
                      </div>
                      <button
                        onClick={() => handleRemoveFromWhitelist(entry.id)}
                        className="text-red-400 hover:text-red-300 transition-colors p-2 hover:bg-red-400/10 rounded-lg"
                      >
                        <Trash2 className="w-4 h-4" />
                      </button>
                    </div>
                  </div>
                </motion.div>
              ))}
              
              {filteredEntries.length === 0 && (
                <div className="text-center py-8">
                  <Users className="w-12 h-12 text-white/40 mx-auto mb-4" />
                  <p className="text-white/60">No whitelisted addresses found</p>
                </div>
              )}
            </div>
          </div>
        </motion.div>
      )}
    </div>
  );
};

export default WhitelistManagement;
