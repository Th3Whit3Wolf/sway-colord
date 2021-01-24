%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}
%define _source_payload w7.xzdio
%define _binary_payload w7.xzdio

Name: sway-colord
Summary: WIP daemon for sway to automatically change light & dark themes
Version: @@VERSION@@
Release: @@RELEASE@@
License: Apache
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

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