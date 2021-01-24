%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: sway-colord
Summary: a daemon for sway to automatically change light and dark themes based on the time of day.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: apache
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/Th3Whit3Wolf/sway-colord

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
