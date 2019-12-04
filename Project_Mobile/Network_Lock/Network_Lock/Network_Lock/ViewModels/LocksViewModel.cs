using System;
using System.Collections.ObjectModel;
using System.Diagnostics;
using System.Threading.Tasks;

using Xamarin.Forms;

using Network_Lock.Models;
using Network_Lock.Views;

namespace Network_Lock.ViewModels
{
    public class LocksViewModel : BaseViewModel
    {
        public ObservableCollection<Lock> Locks { get; set; }
        public Command LoadLocksCommand { get; set; }

        public LocksViewModel()
        {
            Title = "Browse";
            Locks = new ObservableCollection<Lock>();
            LoadLocksCommand = new Command(async () => await ExecuteLoadLocksCommand());


            // TODO: create NewLockPage or AddLockPage
            // MessagingCenter.Subscribe<NewLockPage, Lock>(this, "AddLock", async (obj, l) =>
            {
                // var newLock = l as Lock;
                // Locks.Add(newLock);
                // await DataStore.AddItemAsync(newLock);
            } //);
        }

        async Task ExecuteLoadLocksCommand()
        {
            if (IsBusy)
                return;

            IsBusy = true;

            try
            {
                Locks.Clear();
                // TODO: DataStore.GetLocksAsync(bool)
                //var locks = await DataStore.GetLocksAsync(true); 
                var locks = await DataStore.GetItemsAsync(true); 
                //foreach (var l in locks)
                {
                    //Locks.Add(l);
                }
            }
            catch (Exception ex)
            {
                Debug.WriteLine(ex);
            }
            finally
            {
                IsBusy = false;
            }
        }
    }
}
