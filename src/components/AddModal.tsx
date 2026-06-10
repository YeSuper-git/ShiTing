import { useState } from 'react';

interface AddModalProps {
  isOpen: boolean;
  onClose: () => void;
  onAdd: (type: string, data: any) => void;
}

export function AddModal({ isOpen, onClose, onAdd }: AddModalProps) {
  const [selectedType, setSelectedType] = useState<string>('');
  const [formData, setFormData] = useState({
    name: '',
    recordType: '月卡',
    purchaseDate: '',
    startDate: '',
    expireDate: '',
    amount: '',
    note: '',
  });

  if (!isOpen) return null;

  const types = [
    { id: 'game', icon: '🎮', name: '游戏', desc: '月卡、通行证、会员' },
    { id: 'hardware', icon: '💻', name: '硬件质保', desc: '电子产品、家电' },
    { id: 'consumable', icon: '🏠', name: '家用耗材', desc: '滤芯、电池、清洁用品' },
    { id: 'medicine', icon: '💊', name: '药品', desc: '家庭常备药' },
  ];

  const handleSubmit = () => {
    if (!selectedType || !formData.name) return;
    onAdd(selectedType, formData);
    onClose();
    setFormData({ name: '', recordType: '月卡', purchaseDate: '', startDate: '', expireDate: '', amount: '', note: '' });
    setSelectedType('');
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2 className="modal-title">添加记录</h2>
          <button className="modal-close" onClick={onClose}>✕</button>
        </div>
        
        <div className="modal-body">
          {!selectedType ? (
            <>
              <p className="modal-desc">选择要添加的记录类型</p>
              <div className="type-grid">
                {types.map((type) => (
                  <div
                    key={type.id}
                    className="type-card"
                    onClick={() => setSelectedType(type.id)}
                  >
                    <div className="type-icon">{type.icon}</div>
                    <div className="type-name">{type.name}</div>
                    <div className="type-desc">{type.desc}</div>
                  </div>
                ))}
              </div>
            </>
          ) : (
            <div className="form">
              <div className="form-group">
                <label className="form-label">名称</label>
                <input
                  className="form-input"
                  placeholder={selectedType === 'game' ? '游戏名称' : '商品名称'}
                  value={formData.name}
                  onChange={(e) => setFormData({ ...formData, name: e.target.value })}
                />
              </div>

              {selectedType === 'game' && (
                <div className="form-group">
                  <label className="form-label">类型</label>
                  <select
                    className="form-input"
                    value={formData.recordType}
                    onChange={(e) => setFormData({ ...formData, recordType: e.target.value })}
                  >
                    <option value="月卡">月卡</option>
                    <option value="大月卡">大月卡</option>
                    <option value="通行证">通行证</option>
                    <option value="会员">会员</option>
                  </select>
                </div>
              )}

              <div className="form-group">
                <label className="form-label">购买日期</label>
                <input
                  type="date"
                  className="form-input"
                  value={formData.purchaseDate}
                  onChange={(e) => setFormData({ ...formData, purchaseDate: e.target.value })}
                />
              </div>

              {selectedType === 'game' && (
                <>
                  <div className="form-group">
                    <label className="form-label">生效日期</label>
                    <input
                      type="date"
                      className="form-input"
                      value={formData.startDate}
                      onChange={(e) => setFormData({ ...formData, startDate: e.target.value })}
                    />
                  </div>
                  <div className="form-group">
                    <label className="form-label">到期日期</label>
                    <input
                      type="date"
                      className="form-input"
                      value={formData.expireDate}
                      onChange={(e) => setFormData({ ...formData, expireDate: e.target.value })}
                    />
                  </div>
                  <div className="form-group">
                    <label className="form-label">消费金额 (元)</label>
                    <input
                      type="number"
                      className="form-input"
                      placeholder="30"
                      value={formData.amount}
                      onChange={(e) => setFormData({ ...formData, amount: e.target.value })}
                    />
                  </div>
                </>
              )}

              <div className="form-group">
                <label className="form-label">备注</label>
                <textarea
                  className="form-input form-textarea"
                  placeholder="可选备注..."
                  value={formData.note}
                  onChange={(e) => setFormData({ ...formData, note: e.target.value })}
                />
              </div>

              <div className="form-actions">
                <button className="btn btn-ghost" onClick={() => setSelectedType('')}>返回</button>
                <button className="btn btn-primary" onClick={handleSubmit}>添加</button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
