using Network_Lock.Models;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Network_Lock.Services
{
    public class MockLockDataStore : IDataStore<Lock>
    {
        readonly List<Lock> locks;

        public MockLockDataStore()
        {
            locks = new List<Lock>()
            {
                new Lock { Id = Guid.NewGuid().ToString(), 
                    Name = "First Lock"
                    },
                 new Lock { Id = Guid.NewGuid().ToString(),
                    Name = "Second Lock"
                    },
                 new Lock { Id = Guid.NewGuid().ToString(),
                    Name = "Lock the 3rd"
                    },
                new Lock { Id = Guid.NewGuid().ToString(),
                    Name = "The Lockinator"
                    }
            };
        }

        public async Task<bool> AddItemAsync(Lock l)
        {
            locks.Add(l);
            return await Task.FromResult(true);
        }

        public async Task<bool> UpdateItemAsync(Lock l)
        {
            var oldItem = locks.Where( (Lock arg) => arg.Id == l.Id).FirstOrDefault();
            locks.Remove(oldItem);
            locks.Add(l);

            return await Task.FromResult(true);
        }

        public async Task<bool> DeleteItemAsync(string id)
        {
            var oldItem = locks.Where((Lock arg) => arg.Id == id).FirstOrDefault();
            locks.Remove(oldItem);

            return await Task.FromResult(true);
        }

        public async Task<Lock> GetItemAsync(string id)
        {
            return await Task.FromResult(locks.FirstOrDefault(s => s.Id == id));
        }

        public async Task<IEnumerable<Lock>> GetItemsAsync(bool forceRefresh = false)
        {
            return await Task.FromResult(locks);
        }
    }
}