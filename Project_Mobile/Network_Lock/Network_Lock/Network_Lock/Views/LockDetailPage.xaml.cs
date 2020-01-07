using System;
using System.ComponentModel;
using Xamarin.Forms;
using Xamarin.Forms.Xaml;

using Network_Lock.Models;
using Network_Lock.ViewModels;

namespace Network_Lock.Views
{
    // Learn more about making custom code visible in the Xamarin.Forms previewer
    // by visiting https://aka.ms/xamarinforms-previewer
    [DesignTimeVisible(false)]
    public partial class LockDetailPage : ContentPage
    {
        LockDetailViewModel viewModel;

        public LockDetailPage(LockDetailViewModel viewModel)
        {
            InitializeComponent();

            BindingContext = this.viewModel = viewModel;
        }

        public LockDetailPage()
        {
            InitializeComponent();

            var lock_ = new Lock
            {
                Name = "<No lock found>"
            };

            viewModel = new LockDetailViewModel(lock_);
            BindingContext = viewModel;
        }
    }
}