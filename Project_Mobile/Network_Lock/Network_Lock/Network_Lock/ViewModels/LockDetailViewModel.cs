using System;

using Network_Lock.Models;

namespace Network_Lock.ViewModels
{
    public class LockDetailViewModel : BaseViewModel
    {
        public Lock Lock { get; set; }
        public LockDetailViewModel(Lock newLock = null)
        {
            Title = newLock?.Name;
            Lock = newLock;
        }
    }
}
