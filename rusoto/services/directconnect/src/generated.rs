// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;
use std::io;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Container for the parameters to the AllocateConnectionOnInterconnect operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocateConnectionOnInterconnectRequest {
    /// <p>Bandwidth of the connection.</p> <p>Example: "<i>500Mbps</i>"</p> <p>Default: None</p> <p>Values: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, or 500Mbps</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>Name of the provisioned connection.</p> <p>Example: "<i>500M Connection to AWS</i>"</p> <p>Default: None</p>
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    /// <p>ID of the interconnect on which the connection will be provisioned.</p> <p>Example: dxcon-456abc78</p> <p>Default: None</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
    /// <p>Numeric account Id of the customer for whom the connection will be provisioned.</p> <p>Example: 123443215678</p> <p>Default: None</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
    /// <p>The dedicated VLAN provisioned to the connection.</p> <p>Example: 101</p> <p>Default: None</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Container for the parameters to theHostedConnection operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocateHostedConnectionRequest {
    /// <p>The bandwidth of the connection.</p> <p>Example: <code>500Mbps</code> </p> <p>Default: None</p> <p>Values: 50Mbps, 100Mbps, 200Mbps, 300Mbps, 400Mbps, or 500Mbps</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>The ID of the interconnect or LAG on which the connection will be provisioned.</p> <p>Example: dxcon-456abc78 or dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The name of the provisioned connection.</p> <p>Example: "<code>500M Connection to AWS</code>"</p> <p>Default: None</p>
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    /// <p>The numeric account ID of the customer for whom the connection will be provisioned.</p> <p>Example: 123443215678</p> <p>Default: None</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
    /// <p>The dedicated VLAN provisioned to the hosted connection.</p> <p>Example: 101</p> <p>Default: None</p>
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>Container for the parameters to the AllocatePrivateVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocatePrivateVirtualInterfaceRequest {
    /// <p>The connection ID on which the private virtual interface is provisioned.</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Detailed information for the private virtual interface to be provisioned.</p> <p>Default: None</p>
    #[serde(rename = "newPrivateVirtualInterfaceAllocation")]
    pub new_private_virtual_interface_allocation: NewPrivateVirtualInterfaceAllocation,
    /// <p>The AWS account that will own the new private virtual interface.</p> <p>Default: None</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
}

/// <p>Container for the parameters to the AllocatePublicVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocatePublicVirtualInterfaceRequest {
    /// <p>The connection ID on which the public virtual interface is provisioned.</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Detailed information for the public virtual interface to be provisioned.</p> <p>Default: None</p>
    #[serde(rename = "newPublicVirtualInterfaceAllocation")]
    pub new_public_virtual_interface_allocation: NewPublicVirtualInterfaceAllocation,
    /// <p>The AWS account that will own the new public virtual interface.</p> <p>Default: None</p>
    #[serde(rename = "ownerAccount")]
    pub owner_account: String,
}

/// <p>Container for the parameters to the AssociateConnectionWithLag operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateConnectionWithLagRequest {
    /// <p>The ID of the connection.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the LAG with which to associate the connection.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

/// <p>Container for the parameters to the AssociateHostedConnection operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateHostedConnectionRequest {
    /// <p>The ID of the hosted connection.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the interconnect or the LAG.</p> <p>Example: dxcon-abc123 or dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "parentConnectionId")]
    pub parent_connection_id: String,
}

/// <p>Container for the parameters to the AssociateVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssociateVirtualInterfaceRequest {
    /// <p>The ID of the LAG or connection with which to associate the virtual interface.</p> <p>Example: dxlag-abc123 or dxcon-abc123</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the virtual interface.</p> <p>Example: dxvif-123dfg56</p> <p>Default: None</p>
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

/// <p>A structure containing information about a BGP peer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BGPPeer {
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The Direct Connection endpoint which the BGP peer terminates on.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "bgpPeerState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peer_state: Option<String>,
    #[serde(rename = "bgpStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_status: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
}

/// <p>Container for the parameters to the ConfirmConnection operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmConnectionRequest {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

/// <p>The response received when ConfirmConnection is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmConnectionResponse {
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
}

/// <p>Container for the parameters to the ConfirmPrivateVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmPrivateVirtualInterfaceRequest {
    /// <p>ID of the direct connect gateway that will be attached to the virtual interface.</p> <p> A direct connect gateway can be managed via the AWS Direct Connect console or the <a>CreateDirectConnectGateway</a> action.</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>ID of the virtual private gateway that will be attached to the virtual interface.</p> <p> A virtual private gateway can be managed via the Amazon Virtual Private Cloud (VPC) console or the <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html">EC2 CreateVpnGateway</a> action.</p> <p>Default: None</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

/// <p>The response received when ConfirmPrivateVirtualInterface is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmPrivateVirtualInterfaceResponse {
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

/// <p>Container for the parameters to the ConfirmPublicVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfirmPublicVirtualInterfaceRequest {
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

/// <p>The response received when ConfirmPublicVirtualInterface is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfirmPublicVirtualInterfaceResponse {
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

/// <p>A connection represents the physical network connection between the AWS Direct Connect location and the customer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connection {
    /// <p>Deprecated in favor of awsDeviceV2.</p> <p>The Direct Connection endpoint which the physical connection terminates on.</p>
    #[serde(rename = "awsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    /// <p>The Direct Connection endpoint which the physical connection terminates on.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>Bandwidth of the connection.</p> <p>Example: 1Gbps (for regular connections), or 500Mbps (for hosted connections)</p> <p>Default: None</p>
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "connectionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_name: Option<String>,
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The time of the most recent call to <a>DescribeLoa</a> for this connection.</p>
    #[serde(rename = "loaIssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_issue_time: Option<f64>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The AWS account that will own the new connection.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// <p>The name of the AWS Direct Connect service provider associated with the connection.</p>
    #[serde(rename = "partnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "vlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i64>,
}

/// <p>A structure containing a list of connections.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connections {
    /// <p>A list of connections.</p>
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
}

/// <p>Container for the parameters to the CreateBGPPeer operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBGPPeerRequest {
    /// <p>Detailed information for the BGP peer to be created.</p> <p>Default: None</p>
    #[serde(rename = "newBGPPeer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_bgp_peer: Option<NewBGPPeer>,
    /// <p>The ID of the virtual interface on which the BGP peer will be provisioned.</p> <p>Example: dxvif-456abc78</p> <p>Default: None</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

/// <p>The response received when CreateBGPPeer is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateBGPPeerResponse {
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

/// <p>Container for the parameters to the CreateConnection operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateConnectionRequest {
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    #[serde(rename = "connectionName")]
    pub connection_name: String,
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    #[serde(rename = "location")]
    pub location: String,
}

/// <p>Container for the parameters to the CreateDirectConnectGatewayAssociation operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDirectConnectGatewayAssociationRequest {
    /// <p>The ID of the direct connect gateway.</p> <p>Example: "abcd1234-dcba-5678-be23-cdef9876ab45"</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>The ID of the virtual private gateway.</p> <p>Example: "vgw-abc123ef"</p> <p>Default: None</p>
    #[serde(rename = "virtualGatewayId")]
    pub virtual_gateway_id: String,
}

/// <p>Container for the response from the CreateDirectConnectGatewayAssociation API call</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDirectConnectGatewayAssociationResult {
    /// <p>The direct connect gateway association to be created.</p>
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

/// <p>Container for the parameters to the CreateDirectConnectGateway operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDirectConnectGatewayRequest {
    /// <p>The autonomous system number (ASN) for Border Gateway Protocol (BGP) to be configured on the Amazon side of the connection. The ASN must be in the private range of 64,512 to 65,534 or 4,200,000,000 to 4,294,967,294 </p> <p>Example: 65200</p> <p>Default: 64512</p>
    #[serde(rename = "amazonSideAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    /// <p>The name of the direct connect gateway.</p> <p>Example: "My direct connect gateway"</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayName")]
    pub direct_connect_gateway_name: String,
}

/// <p>Container for the response from the CreateDirectConnectGateway API call</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDirectConnectGatewayResult {
    /// <p>The direct connect gateway to be created.</p>
    #[serde(rename = "directConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

/// <p>Container for the parameters to the CreateInterconnect operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInterconnectRequest {
    /// <p>The port bandwidth</p> <p>Example: 1Gbps</p> <p>Default: None</p> <p>Available values: 1Gbps,10Gbps</p>
    #[serde(rename = "bandwidth")]
    pub bandwidth: String,
    /// <p>The name of the interconnect.</p> <p>Example: "<i>1G Interconnect to AWS</i>"</p> <p>Default: None</p>
    #[serde(rename = "interconnectName")]
    pub interconnect_name: String,
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>Where the interconnect is located</p> <p>Example: EqSV5</p> <p>Default: None</p>
    #[serde(rename = "location")]
    pub location: String,
}

/// <p>Container for the parameters to the CreateLag operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLagRequest {
    /// <p>The ID of an existing connection to migrate to the LAG.</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The bandwidth of the individual physical connections bundled by the LAG.</p> <p>Default: None</p> <p>Available values: 1Gbps, 10Gbps</p>
    #[serde(rename = "connectionsBandwidth")]
    pub connections_bandwidth: String,
    /// <p>The name of the LAG.</p> <p>Example: "<code>3x10G LAG to AWS</code>"</p> <p>Default: None</p>
    #[serde(rename = "lagName")]
    pub lag_name: String,
    /// <p>The AWS Direct Connect location in which the LAG should be allocated.</p> <p>Example: EqSV5</p> <p>Default: None</p>
    #[serde(rename = "location")]
    pub location: String,
    /// <p>The number of physical connections initially provisioned and bundled by the LAG.</p> <p>Default: None</p>
    #[serde(rename = "numberOfConnections")]
    pub number_of_connections: i64,
}

/// <p>Container for the parameters to the CreatePrivateVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePrivateVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Detailed information for the private virtual interface to be created.</p> <p>Default: None</p>
    #[serde(rename = "newPrivateVirtualInterface")]
    pub new_private_virtual_interface: NewPrivateVirtualInterface,
}

/// <p>Container for the parameters to the CreatePublicVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePublicVirtualInterfaceRequest {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>Detailed information for the public virtual interface to be created.</p> <p>Default: None</p>
    #[serde(rename = "newPublicVirtualInterface")]
    pub new_public_virtual_interface: NewPublicVirtualInterface,
}

/// <p>Container for the parameters to the DeleteBGPPeer operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBGPPeerRequest {
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>The ID of the virtual interface from which the BGP peer will be deleted.</p> <p>Example: dxvif-456abc78</p> <p>Default: None</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

/// <p>The response received when DeleteBGPPeer is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteBGPPeerResponse {
    #[serde(rename = "virtualInterface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface: Option<VirtualInterface>,
}

/// <p>Container for the parameters to the DeleteConnection operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConnectionRequest {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

/// <p>Container for the parameters to the DeleteDirectConnectGatewayAssociation operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDirectConnectGatewayAssociationRequest {
    /// <p>The ID of the direct connect gateway.</p> <p>Example: "abcd1234-dcba-5678-be23-cdef9876ab45"</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
    /// <p>The ID of the virtual private gateway.</p> <p>Example: "vgw-abc123ef"</p> <p>Default: None</p>
    #[serde(rename = "virtualGatewayId")]
    pub virtual_gateway_id: String,
}

/// <p>Container for the response from the DeleteDirectConnectGatewayAssociation API call</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDirectConnectGatewayAssociationResult {
    /// <p>The direct connect gateway association to be deleted.</p>
    #[serde(rename = "directConnectGatewayAssociation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_association: Option<DirectConnectGatewayAssociation>,
}

/// <p>Container for the parameters to the DeleteDirectConnectGateway operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDirectConnectGatewayRequest {
    /// <p>The ID of the direct connect gateway.</p> <p>Example: "abcd1234-dcba-5678-be23-cdef9876ab45"</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayId")]
    pub direct_connect_gateway_id: String,
}

/// <p>Container for the response from the DeleteDirectConnectGateway API call</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDirectConnectGatewayResult {
    /// <p>The direct connect gateway to be deleted.</p>
    #[serde(rename = "directConnectGateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway: Option<DirectConnectGateway>,
}

/// <p>Container for the parameters to the DeleteInterconnect operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInterconnectRequest {
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
}

/// <p>The response received when DeleteInterconnect is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteInterconnectResponse {
    #[serde(rename = "interconnectState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_state: Option<String>,
}

/// <p>Container for the parameters to the DeleteLag operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLagRequest {
    /// <p>The ID of the LAG to delete.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

/// <p>Container for the parameters to the DeleteVirtualInterface operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVirtualInterfaceRequest {
    #[serde(rename = "virtualInterfaceId")]
    pub virtual_interface_id: String,
}

/// <p>The response received when DeleteVirtualInterface is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteVirtualInterfaceResponse {
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
}

/// <p>Container for the parameters to the DescribeConnectionLoa operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConnectionLoaRequest {
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    /// <p>The name of the APN partner or service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

/// <p>The response received when DescribeConnectionLoa is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConnectionLoaResponse {
    #[serde(rename = "loa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

/// <p>Container for the parameters to the DescribeConnectionsOnInterconnect operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConnectionsOnInterconnectRequest {
    /// <p>ID of the interconnect on which a list of connection is provisioned.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
}

/// <p>Container for the parameters to the DescribeConnections operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConnectionsRequest {
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
}

/// <p>Container for the parameters to the DescribeDirectConnectGatewayAssociations operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectConnectGatewayAssociationsRequest {
    /// <p>The ID of the direct connect gateway.</p> <p>Example: "abcd1234-dcba-5678-be23-cdef9876ab45"</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of direct connect gateway associations to return per page.</p> <p>Example: 15</p> <p>Default: None</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token provided in the previous describe result to retrieve the next page of the result.</p> <p>Default: None</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the virtual private gateway.</p> <p>Example: "vgw-abc123ef"</p> <p>Default: None</p>
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
}

/// <p>Container for the response from the DescribeDirectConnectGatewayAssociations API call</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDirectConnectGatewayAssociationsResult {
    /// <p>Information about the direct connect gateway associations.</p>
    #[serde(rename = "directConnectGatewayAssociations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_associations: Option<Vec<DirectConnectGatewayAssociation>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Container for the parameters to the DescribeDirectConnectGatewayAttachments operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectConnectGatewayAttachmentsRequest {
    /// <p>The ID of the direct connect gateway.</p> <p>Example: "abcd1234-dcba-5678-be23-cdef9876ab45"</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of direct connect gateway attachments to return per page.</p> <p>Example: 15</p> <p>Default: None</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token provided in the previous describe result to retrieve the next page of the result.</p> <p>Default: None</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the virtual interface.</p> <p>Example: "dxvif-abc123ef"</p> <p>Default: None</p>
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

/// <p>Container for the response from the DescribeDirectConnectGatewayAttachments API call</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDirectConnectGatewayAttachmentsResult {
    /// <p>Information about the direct connect gateway attachments.</p>
    #[serde(rename = "directConnectGatewayAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_attachments: Option<Vec<DirectConnectGatewayAttachment>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Container for the parameters to the DescribeDirectConnectGateways operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDirectConnectGatewaysRequest {
    /// <p>The ID of the direct connect gateway.</p> <p>Example: "abcd1234-dcba-5678-be23-cdef9876ab45"</p> <p>Default: None</p>
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    /// <p>The maximum number of direct connect gateways to return per page.</p> <p>Example: 15</p> <p>Default: None</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token provided in the previous describe result to retrieve the next page of the result.</p> <p>Default: None</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Container for the response from the DescribeDirectConnectGateways API call</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDirectConnectGatewaysResult {
    /// <p>Information about the direct connect gateways.</p>
    #[serde(rename = "directConnectGateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateways: Option<Vec<DirectConnectGateway>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Container for the parameters to the DescribeHostedConnections operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeHostedConnectionsRequest {
    /// <p>The ID of the interconnect or LAG on which the hosted connections are provisioned.</p> <p>Example: dxcon-abc123 or dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
}

/// <p>Container for the parameters to the DescribeInterconnectLoa operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInterconnectLoaRequest {
    #[serde(rename = "interconnectId")]
    pub interconnect_id: String,
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    /// <p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

/// <p>The response received when DescribeInterconnectLoa is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeInterconnectLoaResponse {
    #[serde(rename = "loa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa: Option<Loa>,
}

/// <p>Container for the parameters to the DescribeInterconnects operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeInterconnectsRequest {
    #[serde(rename = "interconnectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_id: Option<String>,
}

/// <p>Container for the parameters to the DescribeLags operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLagsRequest {
    /// <p>The ID of the LAG.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
}

/// <p>Container for the parameters to the DescribeLoa operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLoaRequest {
    /// <p>The ID of a connection, LAG, or interconnect for which to get the LOA-CFA information.</p> <p>Example: dxcon-abc123 or dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>A standard media type indicating the content type of the LOA-CFA document. Currently, the only supported value is "application/pdf".</p> <p>Default: application/pdf</p>
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
    /// <p>The name of the service provider who establishes connectivity on your behalf. If you supply this parameter, the LOA-CFA lists the provider name alongside your company name as the requester of the cross connect.</p> <p>Default: None</p>
    #[serde(rename = "providerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_name: Option<String>,
}

/// <p>Container for the parameters to the DescribeTags operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTagsRequest {
    /// <p>The Amazon Resource Names (ARNs) of the Direct Connect resources.</p>
    #[serde(rename = "resourceArns")]
    pub resource_arns: Vec<String>,
}

/// <p>The response received when DescribeTags is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTagsResponse {
    /// <p>Information about the tags.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<Vec<ResourceTag>>,
}

/// <p>Container for the parameters to the DescribeVirtualInterfaces operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeVirtualInterfacesRequest {
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
}

/// <p>A direct connect gateway is an intermediate object that enables you to connect virtual interfaces and virtual private gateways.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectConnectGateway {
    /// <p>The autonomous system number (ASN) for the Amazon side of the connection.</p>
    #[serde(rename = "amazonSideAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "directConnectGatewayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_name: Option<String>,
    #[serde(rename = "directConnectGatewayState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_state: Option<String>,
    /// <p>The AWS account ID of the owner of the direct connect gateway.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "stateChangeError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
}

/// <p>The association between a direct connect gateway and virtual private gateway.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectConnectGatewayAssociation {
    #[serde(rename = "associationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_state: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "stateChangeError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    /// <p>The AWS account ID of the owner of the virtual private gateway.</p>
    #[serde(rename = "virtualGatewayOwnerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_owner_account: Option<String>,
    #[serde(rename = "virtualGatewayRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_region: Option<String>,
}

/// <p>The association between a direct connect gateway and virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DirectConnectGatewayAttachment {
    #[serde(rename = "attachmentState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_state: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "stateChangeError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_error: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
    /// <p>The AWS account ID of the owner of the virtual interface.</p>
    #[serde(rename = "virtualInterfaceOwnerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_owner_account: Option<String>,
    #[serde(rename = "virtualInterfaceRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_region: Option<String>,
}

/// <p>Container for the parameters to the DisassociateConnectionFromLag operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateConnectionFromLagRequest {
    /// <p>The ID of the connection to disassociate from the LAG.</p> <p>Example: dxcon-abc123</p> <p>Default: None</p>
    #[serde(rename = "connectionId")]
    pub connection_id: String,
    /// <p>The ID of the LAG.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
}

/// <p>An interconnect is a connection that can host other connections.</p> <p>Like a standard AWS Direct Connect connection, an interconnect represents the physical connection between an AWS Direct Connect partner's network and a specific Direct Connect location. An AWS Direct Connect partner who owns an interconnect can provision hosted connections on the interconnect for their end customers, thereby providing the end customers with connectivity to AWS services.</p> <p>The resources of the interconnect, including bandwidth and VLAN numbers, are shared by all of the hosted connections on the interconnect, and the owner of the interconnect determines how these resources are assigned.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Interconnect {
    /// <p>Deprecated in favor of awsDeviceV2.</p> <p>The Direct Connection endpoint which the physical connection terminates on.</p>
    #[serde(rename = "awsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    /// <p>The Direct Connection endpoint which the physical connection terminates on.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<String>,
    #[serde(rename = "interconnectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_id: Option<String>,
    #[serde(rename = "interconnectName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_name: Option<String>,
    #[serde(rename = "interconnectState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnect_state: Option<String>,
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The time of the most recent call to DescribeInterconnectLoa for this Interconnect.</p>
    #[serde(rename = "loaIssueTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_issue_time: Option<f64>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>A structure containing a list of interconnects.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Interconnects {
    /// <p>A list of interconnects.</p>
    #[serde(rename = "interconnects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interconnects: Option<Vec<Interconnect>>,
}

/// <p>Describes a link aggregation group (LAG). A LAG is a connection that uses the Link Aggregation Control Protocol (LACP) to logically aggregate a bundle of physical connections. Like an interconnect, it can host other connections. All connections in a LAG must terminate on the same physical AWS Direct Connect endpoint, and must be the same bandwidth.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Lag {
    /// <p><p>Indicates whether the LAG can host other connections.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    #[serde(rename = "allowsHostedConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_hosted_connections: Option<bool>,
    /// <p>Deprecated in favor of awsDeviceV2.</p> <p>The AWS Direct Connection endpoint that hosts the LAG.</p>
    #[serde(rename = "awsDevice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device: Option<String>,
    /// <p>The AWS Direct Connection endpoint that hosts the LAG.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    /// <p>A list of connections bundled by this LAG.</p>
    #[serde(rename = "connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p>The individual bandwidth of the physical connections bundled by the LAG.</p> <p>Available values: 1Gbps, 10Gbps</p>
    #[serde(rename = "connectionsBandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections_bandwidth: Option<String>,
    #[serde(rename = "lagId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_id: Option<String>,
    /// <p>The name of the LAG.</p>
    #[serde(rename = "lagName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_name: Option<String>,
    #[serde(rename = "lagState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_state: Option<String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The minimum number of physical connections that must be operational for the LAG itself to be operational. If the number of operational connections drops below this setting, the LAG state changes to <code>down</code>. This value can help to ensure that a LAG is not overutilized if a significant number of its bundled connections go down.</p>
    #[serde(rename = "minimumLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_links: Option<i64>,
    /// <p>The number of physical connections bundled by the LAG, up to a maximum of 10.</p>
    #[serde(rename = "numberOfConnections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_connections: Option<i64>,
    /// <p>The owner of the LAG.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>A structure containing a list of LAGs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Lags {
    /// <p>A list of LAGs.</p>
    #[serde(rename = "lags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lags: Option<Vec<Lag>>,
}

/// <p>A structure containing the Letter of Authorization - Connecting Facility Assignment (LOA-CFA) for a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Loa {
    #[serde(rename = "loaContent")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub loa_content: Option<Vec<u8>>,
    #[serde(rename = "loaContentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loa_content_type: Option<String>,
}

/// <p>An AWS Direct Connect location where connections and interconnects can be requested.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Location {
    /// <p>The code used to indicate the AWS Direct Connect location.</p>
    #[serde(rename = "locationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_code: Option<String>,
    /// <p>The name of the AWS Direct Connect location. The name includes the colocation partner name and the physical site of the lit building.</p>
    #[serde(rename = "locationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    /// <p>The AWS region where the AWS Direct connect location is located.</p> <p>Example: us-east-1</p> <p>Default: None</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// <p>A location is a network facility where AWS Direct Connect routers are available to be connected. Generally, these are colocation hubs where many network providers have equipment, and where cross connects can be delivered. Locations include a name and facility code, and must be provided when creating a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Locations {
    /// <p>A list of colocation hubs where network providers have equipment. Most regions have multiple locations available.</p>
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

/// <p>A structure containing information about a new BGP peer.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewBGPPeer {
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
}

/// <p>A structure containing information about a new private virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPrivateVirtualInterface {
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename = "asn")]
    pub asn: i64,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>A structure containing information about a private virtual interface that will be provisioned on a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPrivateVirtualInterfaceAllocation {
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename = "asn")]
    pub asn: i64,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>A structure containing information about a new public virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPublicVirtualInterface {
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename = "asn")]
    pub asn: i64,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>A structure containing information about a public virtual interface that will be provisioned on a connection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NewPublicVirtualInterfaceAllocation {
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    #[serde(rename = "asn")]
    pub asn: i64,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "virtualInterfaceName")]
    pub virtual_interface_name: String,
    #[serde(rename = "vlan")]
    pub vlan: i64,
}

/// <p>The tags associated with a Direct Connect resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceTag {
    /// <p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The tags.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>A route filter prefix that the customer can advertise through Border Gateway Protocol (BGP) over a public virtual interface.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RouteFilterPrefix {
    /// <p>CIDR notation for the advertised route. Multiple routes are separated by commas.</p> <p>IPv6 CIDRs must be at least a /64 or shorter</p> <p>Example: 10.10.10.0/24,10.10.11.0/24,2001:db8::/64</p>
    #[serde(rename = "cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// <p>Information about a tag.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value of the tag.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Container for the parameters to the TagResource operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p> <p>Example: arn:aws:directconnect:us-east-1:123456789012:dxcon/dxcon-fg5678gh</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The list of tags to add.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

/// <p>The response received when TagResource is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Container for the parameters to the UntagResource operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the Direct Connect resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The list of tag keys to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>The response received when UntagResource is called.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>Container for the parameters to the UpdateLag operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLagRequest {
    /// <p>The ID of the LAG to update.</p> <p>Example: dxlag-abc123</p> <p>Default: None</p>
    #[serde(rename = "lagId")]
    pub lag_id: String,
    /// <p>The name for the LAG.</p> <p>Example: "<code>3x10G LAG to AWS</code>"</p> <p>Default: None</p>
    #[serde(rename = "lagName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lag_name: Option<String>,
    /// <p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p> <p>Default: None</p>
    #[serde(rename = "minimumLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_links: Option<i64>,
}

/// <p>You can create one or more AWS Direct Connect private virtual interfaces linking to your virtual private gateway.</p> <p>Virtual private gateways can be managed using the Amazon Virtual Private Cloud (Amazon VPC) console or the <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html">Amazon EC2 CreateVpnGateway action</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualGateway {
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualGatewayState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_state: Option<String>,
}

/// <p>A structure containing a list of virtual private gateways.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualGateways {
    /// <p>A list of virtual private gateways.</p>
    #[serde(rename = "virtualGateways")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateways: Option<Vec<VirtualGateway>>,
}

/// <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualInterface {
    #[serde(rename = "addressFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_family: Option<String>,
    #[serde(rename = "amazonAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_address: Option<String>,
    /// <p>The autonomous system number (ASN) for the Amazon side of the connection.</p>
    #[serde(rename = "amazonSideAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_side_asn: Option<i64>,
    #[serde(rename = "asn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "authKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_key: Option<String>,
    /// <p>The Direct Connection endpoint which the virtual interface terminates on.</p>
    #[serde(rename = "awsDeviceV2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_device_v2: Option<String>,
    #[serde(rename = "bgpPeers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bgp_peers: Option<Vec<BGPPeer>>,
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "customerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<String>,
    /// <p>Information for generating the customer router configuration.</p>
    #[serde(rename = "customerRouterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_router_config: Option<String>,
    #[serde(rename = "directConnectGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_connect_gateway_id: Option<String>,
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The AWS account that will own the new virtual interface.</p>
    #[serde(rename = "ownerAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    /// <p>The AWS region where the virtual interface is located.</p> <p>Example: us-east-1</p> <p>Default: None</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "routeFilterPrefixes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_filter_prefixes: Option<Vec<RouteFilterPrefix>>,
    #[serde(rename = "virtualGatewayId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_gateway_id: Option<String>,
    #[serde(rename = "virtualInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_id: Option<String>,
    #[serde(rename = "virtualInterfaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_name: Option<String>,
    #[serde(rename = "virtualInterfaceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_state: Option<String>,
    #[serde(rename = "virtualInterfaceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interface_type: Option<String>,
    #[serde(rename = "vlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vlan: Option<i64>,
}

/// <p>A structure containing a list of virtual interfaces.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VirtualInterfaces {
    /// <p>A list of virtual interfaces.</p>
    #[serde(rename = "virtualInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_interfaces: Option<Vec<VirtualInterface>>,
}

/// Errors returned by AllocateConnectionOnInterconnect
#[derive(Debug, PartialEq)]
pub enum AllocateConnectionOnInterconnectError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocateConnectionOnInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocateConnectionOnInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocateConnectionOnInterconnectError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return AllocateConnectionOnInterconnectError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AllocateConnectionOnInterconnectError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return AllocateConnectionOnInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocateConnectionOnInterconnectError {
    fn from(err: serde_json::error::Error) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocateConnectionOnInterconnectError {
    fn from(err: CredentialsError) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocateConnectionOnInterconnectError {
    fn from(err: HttpDispatchError) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocateConnectionOnInterconnectError {
    fn from(err: io::Error) -> AllocateConnectionOnInterconnectError {
        AllocateConnectionOnInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocateConnectionOnInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocateConnectionOnInterconnectError {
    fn description(&self) -> &str {
        match *self {
            AllocateConnectionOnInterconnectError::DirectConnectClient(ref cause) => cause,
            AllocateConnectionOnInterconnectError::DirectConnectServer(ref cause) => cause,
            AllocateConnectionOnInterconnectError::Validation(ref cause) => cause,
            AllocateConnectionOnInterconnectError::Credentials(ref err) => err.description(),
            AllocateConnectionOnInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocateConnectionOnInterconnectError::ParseError(ref cause) => cause,
            AllocateConnectionOnInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AllocateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AllocateHostedConnectionError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocateHostedConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocateHostedConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocateHostedConnectionError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return AllocateHostedConnectionError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AllocateHostedConnectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AllocateHostedConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocateHostedConnectionError {
    fn from(err: serde_json::error::Error) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocateHostedConnectionError {
    fn from(err: CredentialsError) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocateHostedConnectionError {
    fn from(err: HttpDispatchError) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocateHostedConnectionError {
    fn from(err: io::Error) -> AllocateHostedConnectionError {
        AllocateHostedConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocateHostedConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocateHostedConnectionError {
    fn description(&self) -> &str {
        match *self {
            AllocateHostedConnectionError::DirectConnectClient(ref cause) => cause,
            AllocateHostedConnectionError::DirectConnectServer(ref cause) => cause,
            AllocateHostedConnectionError::Validation(ref cause) => cause,
            AllocateHostedConnectionError::Credentials(ref err) => err.description(),
            AllocateHostedConnectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocateHostedConnectionError::ParseError(ref cause) => cause,
            AllocateHostedConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AllocatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePrivateVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocatePrivateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocatePrivateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocatePrivateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return AllocatePrivateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AllocatePrivateVirtualInterfaceError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return AllocatePrivateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocatePrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocatePrivateVirtualInterfaceError {
    fn from(err: CredentialsError) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocatePrivateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocatePrivateVirtualInterfaceError {
    fn from(err: io::Error) -> AllocatePrivateVirtualInterfaceError {
        AllocatePrivateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocatePrivateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocatePrivateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            AllocatePrivateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::Validation(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::Credentials(ref err) => err.description(),
            AllocatePrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocatePrivateVirtualInterfaceError::ParseError(ref cause) => cause,
            AllocatePrivateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AllocatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AllocatePublicVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AllocatePublicVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> AllocatePublicVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AllocatePublicVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return AllocatePublicVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AllocatePublicVirtualInterfaceError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return AllocatePublicVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AllocatePublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AllocatePublicVirtualInterfaceError {
    fn from(err: CredentialsError) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AllocatePublicVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AllocatePublicVirtualInterfaceError {
    fn from(err: io::Error) -> AllocatePublicVirtualInterfaceError {
        AllocatePublicVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AllocatePublicVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocatePublicVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            AllocatePublicVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::Validation(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::Credentials(ref err) => err.description(),
            AllocatePublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AllocatePublicVirtualInterfaceError::ParseError(ref cause) => cause,
            AllocatePublicVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateConnectionWithLag
#[derive(Debug, PartialEq)]
pub enum AssociateConnectionWithLagError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateConnectionWithLagError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateConnectionWithLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AssociateConnectionWithLagError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return AssociateConnectionWithLagError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AssociateConnectionWithLagError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AssociateConnectionWithLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateConnectionWithLagError {
    fn from(err: serde_json::error::Error) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateConnectionWithLagError {
    fn from(err: CredentialsError) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateConnectionWithLagError {
    fn from(err: HttpDispatchError) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateConnectionWithLagError {
    fn from(err: io::Error) -> AssociateConnectionWithLagError {
        AssociateConnectionWithLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateConnectionWithLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateConnectionWithLagError {
    fn description(&self) -> &str {
        match *self {
            AssociateConnectionWithLagError::DirectConnectClient(ref cause) => cause,
            AssociateConnectionWithLagError::DirectConnectServer(ref cause) => cause,
            AssociateConnectionWithLagError::Validation(ref cause) => cause,
            AssociateConnectionWithLagError::Credentials(ref err) => err.description(),
            AssociateConnectionWithLagError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateConnectionWithLagError::ParseError(ref cause) => cause,
            AssociateConnectionWithLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateHostedConnection
#[derive(Debug, PartialEq)]
pub enum AssociateHostedConnectionError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateHostedConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateHostedConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AssociateHostedConnectionError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return AssociateHostedConnectionError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AssociateHostedConnectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AssociateHostedConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateHostedConnectionError {
    fn from(err: serde_json::error::Error) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateHostedConnectionError {
    fn from(err: CredentialsError) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateHostedConnectionError {
    fn from(err: HttpDispatchError) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateHostedConnectionError {
    fn from(err: io::Error) -> AssociateHostedConnectionError {
        AssociateHostedConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateHostedConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateHostedConnectionError {
    fn description(&self) -> &str {
        match *self {
            AssociateHostedConnectionError::DirectConnectClient(ref cause) => cause,
            AssociateHostedConnectionError::DirectConnectServer(ref cause) => cause,
            AssociateHostedConnectionError::Validation(ref cause) => cause,
            AssociateHostedConnectionError::Credentials(ref err) => err.description(),
            AssociateHostedConnectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateHostedConnectionError::ParseError(ref cause) => cause,
            AssociateHostedConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by AssociateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum AssociateVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AssociateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> AssociateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return AssociateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return AssociateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return AssociateVirtualInterfaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return AssociateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AssociateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateVirtualInterfaceError {
    fn from(err: CredentialsError) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateVirtualInterfaceError {
    fn from(err: io::Error) -> AssociateVirtualInterfaceError {
        AssociateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            AssociateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            AssociateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            AssociateVirtualInterfaceError::Validation(ref cause) => cause,
            AssociateVirtualInterfaceError::Credentials(ref err) => err.description(),
            AssociateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateVirtualInterfaceError::ParseError(ref cause) => cause,
            AssociateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ConfirmConnection
#[derive(Debug, PartialEq)]
pub enum ConfirmConnectionError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ConfirmConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> ConfirmConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return ConfirmConnectionError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return ConfirmConnectionError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return ConfirmConnectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ConfirmConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ConfirmConnectionError {
    fn from(err: serde_json::error::Error) -> ConfirmConnectionError {
        ConfirmConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmConnectionError {
    fn from(err: CredentialsError) -> ConfirmConnectionError {
        ConfirmConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmConnectionError {
    fn from(err: HttpDispatchError) -> ConfirmConnectionError {
        ConfirmConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfirmConnectionError {
    fn from(err: io::Error) -> ConfirmConnectionError {
        ConfirmConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfirmConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmConnectionError {
    fn description(&self) -> &str {
        match *self {
            ConfirmConnectionError::DirectConnectClient(ref cause) => cause,
            ConfirmConnectionError::DirectConnectServer(ref cause) => cause,
            ConfirmConnectionError::Validation(ref cause) => cause,
            ConfirmConnectionError::Credentials(ref err) => err.description(),
            ConfirmConnectionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmConnectionError::ParseError(ref cause) => cause,
            ConfirmConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ConfirmPrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPrivateVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ConfirmPrivateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> ConfirmPrivateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return ConfirmPrivateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return ConfirmPrivateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ConfirmPrivateVirtualInterfaceError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return ConfirmPrivateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: CredentialsError) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfirmPrivateVirtualInterfaceError {
    fn from(err: io::Error) -> ConfirmPrivateVirtualInterfaceError {
        ConfirmPrivateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfirmPrivateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmPrivateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            ConfirmPrivateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::Validation(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::Credentials(ref err) => err.description(),
            ConfirmPrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmPrivateVirtualInterfaceError::ParseError(ref cause) => cause,
            ConfirmPrivateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ConfirmPublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum ConfirmPublicVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ConfirmPublicVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> ConfirmPublicVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return ConfirmPublicVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return ConfirmPublicVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return ConfirmPublicVirtualInterfaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ConfirmPublicVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ConfirmPublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ConfirmPublicVirtualInterfaceError {
    fn from(err: CredentialsError) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ConfirmPublicVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ConfirmPublicVirtualInterfaceError {
    fn from(err: io::Error) -> ConfirmPublicVirtualInterfaceError {
        ConfirmPublicVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ConfirmPublicVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ConfirmPublicVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            ConfirmPublicVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::Validation(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::Credentials(ref err) => err.description(),
            ConfirmPublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ConfirmPublicVirtualInterfaceError::ParseError(ref cause) => cause,
            ConfirmPublicVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateBGPPeer
#[derive(Debug, PartialEq)]
pub enum CreateBGPPeerError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateBGPPeerError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateBGPPeerError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateBGPPeerError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return CreateBGPPeerError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateBGPPeerError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateBGPPeerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateBGPPeerError {
    fn from(err: serde_json::error::Error) -> CreateBGPPeerError {
        CreateBGPPeerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateBGPPeerError {
    fn from(err: CredentialsError) -> CreateBGPPeerError {
        CreateBGPPeerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateBGPPeerError {
    fn from(err: HttpDispatchError) -> CreateBGPPeerError {
        CreateBGPPeerError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateBGPPeerError {
    fn from(err: io::Error) -> CreateBGPPeerError {
        CreateBGPPeerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateBGPPeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBGPPeerError {
    fn description(&self) -> &str {
        match *self {
            CreateBGPPeerError::DirectConnectClient(ref cause) => cause,
            CreateBGPPeerError::DirectConnectServer(ref cause) => cause,
            CreateBGPPeerError::Validation(ref cause) => cause,
            CreateBGPPeerError::Credentials(ref err) => err.description(),
            CreateBGPPeerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateBGPPeerError::ParseError(ref cause) => cause,
            CreateBGPPeerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateConnection
#[derive(Debug, PartialEq)]
pub enum CreateConnectionError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateConnectionError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return CreateConnectionError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateConnectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateConnectionError {
    fn from(err: serde_json::error::Error) -> CreateConnectionError {
        CreateConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateConnectionError {
    fn from(err: CredentialsError) -> CreateConnectionError {
        CreateConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateConnectionError {
    fn from(err: HttpDispatchError) -> CreateConnectionError {
        CreateConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateConnectionError {
    fn from(err: io::Error) -> CreateConnectionError {
        CreateConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateConnectionError {
    fn description(&self) -> &str {
        match *self {
            CreateConnectionError::DirectConnectClient(ref cause) => cause,
            CreateConnectionError::DirectConnectServer(ref cause) => cause,
            CreateConnectionError::Validation(ref cause) => cause,
            CreateConnectionError::Credentials(ref err) => err.description(),
            CreateConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateConnectionError::ParseError(ref cause) => cause,
            CreateConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDirectConnectGateway
#[derive(Debug, PartialEq)]
pub enum CreateDirectConnectGatewayError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDirectConnectGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDirectConnectGatewayError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateDirectConnectGatewayError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return CreateDirectConnectGatewayError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreateDirectConnectGatewayError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateDirectConnectGatewayError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDirectConnectGatewayError {
    fn from(err: serde_json::error::Error) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectConnectGatewayError {
    fn from(err: CredentialsError) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectConnectGatewayError {
    fn from(err: HttpDispatchError) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDirectConnectGatewayError {
    fn from(err: io::Error) -> CreateDirectConnectGatewayError {
        CreateDirectConnectGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDirectConnectGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectConnectGatewayError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectConnectGatewayError::DirectConnectClient(ref cause) => cause,
            CreateDirectConnectGatewayError::DirectConnectServer(ref cause) => cause,
            CreateDirectConnectGatewayError::Validation(ref cause) => cause,
            CreateDirectConnectGatewayError::Credentials(ref err) => err.description(),
            CreateDirectConnectGatewayError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDirectConnectGatewayError::ParseError(ref cause) => cause,
            CreateDirectConnectGatewayError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateDirectConnectGatewayAssociation
#[derive(Debug, PartialEq)]
pub enum CreateDirectConnectGatewayAssociationError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateDirectConnectGatewayAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateDirectConnectGatewayAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateDirectConnectGatewayAssociationError::DirectConnectClient(
                        String::from(error_message),
                    )
                }
                "DirectConnectServerException" => {
                    return CreateDirectConnectGatewayAssociationError::DirectConnectServer(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return CreateDirectConnectGatewayAssociationError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return CreateDirectConnectGatewayAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDirectConnectGatewayAssociationError {
    fn from(err: serde_json::error::Error) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDirectConnectGatewayAssociationError {
    fn from(err: CredentialsError) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDirectConnectGatewayAssociationError {
    fn from(err: HttpDispatchError) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDirectConnectGatewayAssociationError {
    fn from(err: io::Error) -> CreateDirectConnectGatewayAssociationError {
        CreateDirectConnectGatewayAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDirectConnectGatewayAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDirectConnectGatewayAssociationError {
    fn description(&self) -> &str {
        match *self {
            CreateDirectConnectGatewayAssociationError::DirectConnectClient(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::DirectConnectServer(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::Validation(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::Credentials(ref err) => err.description(),
            CreateDirectConnectGatewayAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDirectConnectGatewayAssociationError::ParseError(ref cause) => cause,
            CreateDirectConnectGatewayAssociationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateInterconnect
#[derive(Debug, PartialEq)]
pub enum CreateInterconnectError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateInterconnectError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return CreateInterconnectError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateInterconnectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateInterconnectError {
    fn from(err: serde_json::error::Error) -> CreateInterconnectError {
        CreateInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateInterconnectError {
    fn from(err: CredentialsError) -> CreateInterconnectError {
        CreateInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateInterconnectError {
    fn from(err: HttpDispatchError) -> CreateInterconnectError {
        CreateInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateInterconnectError {
    fn from(err: io::Error) -> CreateInterconnectError {
        CreateInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInterconnectError {
    fn description(&self) -> &str {
        match *self {
            CreateInterconnectError::DirectConnectClient(ref cause) => cause,
            CreateInterconnectError::DirectConnectServer(ref cause) => cause,
            CreateInterconnectError::Validation(ref cause) => cause,
            CreateInterconnectError::Credentials(ref err) => err.description(),
            CreateInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateInterconnectError::ParseError(ref cause) => cause,
            CreateInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateLag
#[derive(Debug, PartialEq)]
pub enum CreateLagError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateLagError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreateLagError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return CreateLagError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return CreateLagError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreateLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateLagError {
    fn from(err: serde_json::error::Error) -> CreateLagError {
        CreateLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateLagError {
    fn from(err: CredentialsError) -> CreateLagError {
        CreateLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateLagError {
    fn from(err: HttpDispatchError) -> CreateLagError {
        CreateLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateLagError {
    fn from(err: io::Error) -> CreateLagError {
        CreateLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLagError {
    fn description(&self) -> &str {
        match *self {
            CreateLagError::DirectConnectClient(ref cause) => cause,
            CreateLagError::DirectConnectServer(ref cause) => cause,
            CreateLagError::Validation(ref cause) => cause,
            CreateLagError::Credentials(ref err) => err.description(),
            CreateLagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateLagError::ParseError(ref cause) => cause,
            CreateLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePrivateVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePrivateVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreatePrivateVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePrivateVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreatePrivateVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return CreatePrivateVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreatePrivateVirtualInterfaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreatePrivateVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePrivateVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePrivateVirtualInterfaceError {
    fn from(err: CredentialsError) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePrivateVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePrivateVirtualInterfaceError {
    fn from(err: io::Error) -> CreatePrivateVirtualInterfaceError {
        CreatePrivateVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePrivateVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePrivateVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            CreatePrivateVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::Validation(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::Credentials(ref err) => err.description(),
            CreatePrivateVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePrivateVirtualInterfaceError::ParseError(ref cause) => cause,
            CreatePrivateVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreatePublicVirtualInterface
#[derive(Debug, PartialEq)]
pub enum CreatePublicVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreatePublicVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> CreatePublicVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return CreatePublicVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return CreatePublicVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return CreatePublicVirtualInterfaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return CreatePublicVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePublicVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePublicVirtualInterfaceError {
    fn from(err: CredentialsError) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePublicVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePublicVirtualInterfaceError {
    fn from(err: io::Error) -> CreatePublicVirtualInterfaceError {
        CreatePublicVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreatePublicVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePublicVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            CreatePublicVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            CreatePublicVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            CreatePublicVirtualInterfaceError::Validation(ref cause) => cause,
            CreatePublicVirtualInterfaceError::Credentials(ref err) => err.description(),
            CreatePublicVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreatePublicVirtualInterfaceError::ParseError(ref cause) => cause,
            CreatePublicVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteBGPPeer
#[derive(Debug, PartialEq)]
pub enum DeleteBGPPeerError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteBGPPeerError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteBGPPeerError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteBGPPeerError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DeleteBGPPeerError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteBGPPeerError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteBGPPeerError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteBGPPeerError {
    fn from(err: serde_json::error::Error) -> DeleteBGPPeerError {
        DeleteBGPPeerError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteBGPPeerError {
    fn from(err: CredentialsError) -> DeleteBGPPeerError {
        DeleteBGPPeerError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteBGPPeerError {
    fn from(err: HttpDispatchError) -> DeleteBGPPeerError {
        DeleteBGPPeerError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteBGPPeerError {
    fn from(err: io::Error) -> DeleteBGPPeerError {
        DeleteBGPPeerError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteBGPPeerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBGPPeerError {
    fn description(&self) -> &str {
        match *self {
            DeleteBGPPeerError::DirectConnectClient(ref cause) => cause,
            DeleteBGPPeerError::DirectConnectServer(ref cause) => cause,
            DeleteBGPPeerError::Validation(ref cause) => cause,
            DeleteBGPPeerError::Credentials(ref err) => err.description(),
            DeleteBGPPeerError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteBGPPeerError::ParseError(ref cause) => cause,
            DeleteBGPPeerError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteConnectionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteConnectionError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DeleteConnectionError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteConnectionError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteConnectionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteConnectionError {
    fn from(err: serde_json::error::Error) -> DeleteConnectionError {
        DeleteConnectionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteConnectionError {
    fn from(err: CredentialsError) -> DeleteConnectionError {
        DeleteConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteConnectionError {
    fn from(err: HttpDispatchError) -> DeleteConnectionError {
        DeleteConnectionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteConnectionError {
    fn from(err: io::Error) -> DeleteConnectionError {
        DeleteConnectionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConnectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteConnectionError::DirectConnectClient(ref cause) => cause,
            DeleteConnectionError::DirectConnectServer(ref cause) => cause,
            DeleteConnectionError::Validation(ref cause) => cause,
            DeleteConnectionError::Credentials(ref err) => err.description(),
            DeleteConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteConnectionError::ParseError(ref cause) => cause,
            DeleteConnectionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDirectConnectGateway
#[derive(Debug, PartialEq)]
pub enum DeleteDirectConnectGatewayError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDirectConnectGatewayError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDirectConnectGatewayError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteDirectConnectGatewayError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DeleteDirectConnectGatewayError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteDirectConnectGatewayError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteDirectConnectGatewayError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDirectConnectGatewayError {
    fn from(err: serde_json::error::Error) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectConnectGatewayError {
    fn from(err: CredentialsError) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectConnectGatewayError {
    fn from(err: HttpDispatchError) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDirectConnectGatewayError {
    fn from(err: io::Error) -> DeleteDirectConnectGatewayError {
        DeleteDirectConnectGatewayError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDirectConnectGatewayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectConnectGatewayError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectConnectGatewayError::DirectConnectClient(ref cause) => cause,
            DeleteDirectConnectGatewayError::DirectConnectServer(ref cause) => cause,
            DeleteDirectConnectGatewayError::Validation(ref cause) => cause,
            DeleteDirectConnectGatewayError::Credentials(ref err) => err.description(),
            DeleteDirectConnectGatewayError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDirectConnectGatewayError::ParseError(ref cause) => cause,
            DeleteDirectConnectGatewayError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDirectConnectGatewayAssociation
#[derive(Debug, PartialEq)]
pub enum DeleteDirectConnectGatewayAssociationError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteDirectConnectGatewayAssociationError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDirectConnectGatewayAssociationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteDirectConnectGatewayAssociationError::DirectConnectClient(
                        String::from(error_message),
                    )
                }
                "DirectConnectServerException" => {
                    return DeleteDirectConnectGatewayAssociationError::DirectConnectServer(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return DeleteDirectConnectGatewayAssociationError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DeleteDirectConnectGatewayAssociationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: serde_json::error::Error) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: CredentialsError) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: HttpDispatchError) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDirectConnectGatewayAssociationError {
    fn from(err: io::Error) -> DeleteDirectConnectGatewayAssociationError {
        DeleteDirectConnectGatewayAssociationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDirectConnectGatewayAssociationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDirectConnectGatewayAssociationError {
    fn description(&self) -> &str {
        match *self {
            DeleteDirectConnectGatewayAssociationError::DirectConnectClient(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::DirectConnectServer(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::Validation(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::Credentials(ref err) => err.description(),
            DeleteDirectConnectGatewayAssociationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDirectConnectGatewayAssociationError::ParseError(ref cause) => cause,
            DeleteDirectConnectGatewayAssociationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteInterconnect
#[derive(Debug, PartialEq)]
pub enum DeleteInterconnectError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteInterconnectError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DeleteInterconnectError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteInterconnectError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteInterconnectError {
    fn from(err: serde_json::error::Error) -> DeleteInterconnectError {
        DeleteInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteInterconnectError {
    fn from(err: CredentialsError) -> DeleteInterconnectError {
        DeleteInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteInterconnectError {
    fn from(err: HttpDispatchError) -> DeleteInterconnectError {
        DeleteInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteInterconnectError {
    fn from(err: io::Error) -> DeleteInterconnectError {
        DeleteInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInterconnectError {
    fn description(&self) -> &str {
        match *self {
            DeleteInterconnectError::DirectConnectClient(ref cause) => cause,
            DeleteInterconnectError::DirectConnectServer(ref cause) => cause,
            DeleteInterconnectError::Validation(ref cause) => cause,
            DeleteInterconnectError::Credentials(ref err) => err.description(),
            DeleteInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteInterconnectError::ParseError(ref cause) => cause,
            DeleteInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteLag
#[derive(Debug, PartialEq)]
pub enum DeleteLagError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteLagError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteLagError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DeleteLagError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteLagError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteLagError {
    fn from(err: serde_json::error::Error) -> DeleteLagError {
        DeleteLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLagError {
    fn from(err: CredentialsError) -> DeleteLagError {
        DeleteLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLagError {
    fn from(err: HttpDispatchError) -> DeleteLagError {
        DeleteLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLagError {
    fn from(err: io::Error) -> DeleteLagError {
        DeleteLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLagError {
    fn description(&self) -> &str {
        match *self {
            DeleteLagError::DirectConnectClient(ref cause) => cause,
            DeleteLagError::DirectConnectServer(ref cause) => cause,
            DeleteLagError::Validation(ref cause) => cause,
            DeleteLagError::Credentials(ref err) => err.description(),
            DeleteLagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteLagError::ParseError(ref cause) => cause,
            DeleteLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteVirtualInterface
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualInterfaceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteVirtualInterfaceError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteVirtualInterfaceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DeleteVirtualInterfaceError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DeleteVirtualInterfaceError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DeleteVirtualInterfaceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DeleteVirtualInterfaceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteVirtualInterfaceError {
    fn from(err: serde_json::error::Error) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteVirtualInterfaceError {
    fn from(err: CredentialsError) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteVirtualInterfaceError {
    fn from(err: HttpDispatchError) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteVirtualInterfaceError {
    fn from(err: io::Error) -> DeleteVirtualInterfaceError {
        DeleteVirtualInterfaceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteVirtualInterfaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVirtualInterfaceError {
    fn description(&self) -> &str {
        match *self {
            DeleteVirtualInterfaceError::DirectConnectClient(ref cause) => cause,
            DeleteVirtualInterfaceError::DirectConnectServer(ref cause) => cause,
            DeleteVirtualInterfaceError::Validation(ref cause) => cause,
            DeleteVirtualInterfaceError::Credentials(ref err) => err.description(),
            DeleteVirtualInterfaceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteVirtualInterfaceError::ParseError(ref cause) => cause,
            DeleteVirtualInterfaceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeConnectionLoa
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionLoaError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeConnectionLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeConnectionLoaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeConnectionLoaError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeConnectionLoaError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeConnectionLoaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeConnectionLoaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeConnectionLoaError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConnectionLoaError {
    fn from(err: CredentialsError) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConnectionLoaError {
    fn from(err: HttpDispatchError) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConnectionLoaError {
    fn from(err: io::Error) -> DescribeConnectionLoaError {
        DescribeConnectionLoaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConnectionLoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionLoaError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionLoaError::DirectConnectClient(ref cause) => cause,
            DescribeConnectionLoaError::DirectConnectServer(ref cause) => cause,
            DescribeConnectionLoaError::Validation(ref cause) => cause,
            DescribeConnectionLoaError::Credentials(ref err) => err.description(),
            DescribeConnectionLoaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConnectionLoaError::ParseError(ref cause) => cause,
            DescribeConnectionLoaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeConnections
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeConnectionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeConnectionsError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeConnectionsError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeConnectionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeConnectionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeConnectionsError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionsError {
        DescribeConnectionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConnectionsError {
    fn from(err: CredentialsError) -> DescribeConnectionsError {
        DescribeConnectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConnectionsError {
    fn from(err: HttpDispatchError) -> DescribeConnectionsError {
        DescribeConnectionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConnectionsError {
    fn from(err: io::Error) -> DescribeConnectionsError {
        DescribeConnectionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionsError::DirectConnectClient(ref cause) => cause,
            DescribeConnectionsError::DirectConnectServer(ref cause) => cause,
            DescribeConnectionsError::Validation(ref cause) => cause,
            DescribeConnectionsError::Credentials(ref err) => err.description(),
            DescribeConnectionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConnectionsError::ParseError(ref cause) => cause,
            DescribeConnectionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeConnectionsOnInterconnect
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsOnInterconnectError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeConnectionsOnInterconnectError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeConnectionsOnInterconnectError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeConnectionsOnInterconnectError::DirectConnectClient(
                        String::from(error_message),
                    )
                }
                "DirectConnectServerException" => {
                    return DescribeConnectionsOnInterconnectError::DirectConnectServer(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return DescribeConnectionsOnInterconnectError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeConnectionsOnInterconnectError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeConnectionsOnInterconnectError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConnectionsOnInterconnectError {
    fn from(err: CredentialsError) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConnectionsOnInterconnectError {
    fn from(err: HttpDispatchError) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConnectionsOnInterconnectError {
    fn from(err: io::Error) -> DescribeConnectionsOnInterconnectError {
        DescribeConnectionsOnInterconnectError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConnectionsOnInterconnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionsOnInterconnectError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionsOnInterconnectError::DirectConnectClient(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::DirectConnectServer(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::Validation(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::Credentials(ref err) => err.description(),
            DescribeConnectionsOnInterconnectError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConnectionsOnInterconnectError::ParseError(ref cause) => cause,
            DescribeConnectionsOnInterconnectError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDirectConnectGatewayAssociations
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewayAssociationsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDirectConnectGatewayAssociationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> DescribeDirectConnectGatewayAssociationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeDirectConnectGatewayAssociationsError::DirectConnectClient(
                        String::from(error_message),
                    )
                }
                "DirectConnectServerException" => {
                    return DescribeDirectConnectGatewayAssociationsError::DirectConnectServer(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return DescribeDirectConnectGatewayAssociationsError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeDirectConnectGatewayAssociationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: serde_json::error::Error) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: CredentialsError) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: HttpDispatchError) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDirectConnectGatewayAssociationsError {
    fn from(err: io::Error) -> DescribeDirectConnectGatewayAssociationsError {
        DescribeDirectConnectGatewayAssociationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDirectConnectGatewayAssociationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectConnectGatewayAssociationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectConnectGatewayAssociationsError::DirectConnectClient(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::DirectConnectServer(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::Validation(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::Credentials(ref err) => {
                err.description()
            }
            DescribeDirectConnectGatewayAssociationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectConnectGatewayAssociationsError::ParseError(ref cause) => cause,
            DescribeDirectConnectGatewayAssociationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDirectConnectGatewayAttachments
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewayAttachmentsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDirectConnectGatewayAttachmentsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> DescribeDirectConnectGatewayAttachmentsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeDirectConnectGatewayAttachmentsError::DirectConnectClient(
                        String::from(error_message),
                    )
                }
                "DirectConnectServerException" => {
                    return DescribeDirectConnectGatewayAttachmentsError::DirectConnectServer(
                        String::from(error_message),
                    )
                }
                "ValidationException" => {
                    return DescribeDirectConnectGatewayAttachmentsError::Validation(
                        error_message.to_string(),
                    )
                }
                _ => {}
            }
        }
        return DescribeDirectConnectGatewayAttachmentsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: serde_json::error::Error) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: CredentialsError) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: HttpDispatchError) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDirectConnectGatewayAttachmentsError {
    fn from(err: io::Error) -> DescribeDirectConnectGatewayAttachmentsError {
        DescribeDirectConnectGatewayAttachmentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDirectConnectGatewayAttachmentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectConnectGatewayAttachmentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectConnectGatewayAttachmentsError::DirectConnectClient(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::DirectConnectServer(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::Validation(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::Credentials(ref err) => err.description(),
            DescribeDirectConnectGatewayAttachmentsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectConnectGatewayAttachmentsError::ParseError(ref cause) => cause,
            DescribeDirectConnectGatewayAttachmentsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDirectConnectGateways
#[derive(Debug, PartialEq)]
pub enum DescribeDirectConnectGatewaysError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeDirectConnectGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDirectConnectGatewaysError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeDirectConnectGatewaysError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeDirectConnectGatewaysError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeDirectConnectGatewaysError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeDirectConnectGatewaysError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDirectConnectGatewaysError {
    fn from(err: serde_json::error::Error) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDirectConnectGatewaysError {
    fn from(err: CredentialsError) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDirectConnectGatewaysError {
    fn from(err: HttpDispatchError) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDirectConnectGatewaysError {
    fn from(err: io::Error) -> DescribeDirectConnectGatewaysError {
        DescribeDirectConnectGatewaysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDirectConnectGatewaysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDirectConnectGatewaysError {
    fn description(&self) -> &str {
        match *self {
            DescribeDirectConnectGatewaysError::DirectConnectClient(ref cause) => cause,
            DescribeDirectConnectGatewaysError::DirectConnectServer(ref cause) => cause,
            DescribeDirectConnectGatewaysError::Validation(ref cause) => cause,
            DescribeDirectConnectGatewaysError::Credentials(ref err) => err.description(),
            DescribeDirectConnectGatewaysError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDirectConnectGatewaysError::ParseError(ref cause) => cause,
            DescribeDirectConnectGatewaysError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeHostedConnections
#[derive(Debug, PartialEq)]
pub enum DescribeHostedConnectionsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeHostedConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeHostedConnectionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeHostedConnectionsError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeHostedConnectionsError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeHostedConnectionsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeHostedConnectionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeHostedConnectionsError {
    fn from(err: serde_json::error::Error) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeHostedConnectionsError {
    fn from(err: CredentialsError) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeHostedConnectionsError {
    fn from(err: HttpDispatchError) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeHostedConnectionsError {
    fn from(err: io::Error) -> DescribeHostedConnectionsError {
        DescribeHostedConnectionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeHostedConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeHostedConnectionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeHostedConnectionsError::DirectConnectClient(ref cause) => cause,
            DescribeHostedConnectionsError::DirectConnectServer(ref cause) => cause,
            DescribeHostedConnectionsError::Validation(ref cause) => cause,
            DescribeHostedConnectionsError::Credentials(ref err) => err.description(),
            DescribeHostedConnectionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeHostedConnectionsError::ParseError(ref cause) => cause,
            DescribeHostedConnectionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeInterconnectLoa
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectLoaError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeInterconnectLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeInterconnectLoaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeInterconnectLoaError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeInterconnectLoaError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeInterconnectLoaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeInterconnectLoaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeInterconnectLoaError {
    fn from(err: serde_json::error::Error) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInterconnectLoaError {
    fn from(err: CredentialsError) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInterconnectLoaError {
    fn from(err: HttpDispatchError) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInterconnectLoaError {
    fn from(err: io::Error) -> DescribeInterconnectLoaError {
        DescribeInterconnectLoaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInterconnectLoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInterconnectLoaError {
    fn description(&self) -> &str {
        match *self {
            DescribeInterconnectLoaError::DirectConnectClient(ref cause) => cause,
            DescribeInterconnectLoaError::DirectConnectServer(ref cause) => cause,
            DescribeInterconnectLoaError::Validation(ref cause) => cause,
            DescribeInterconnectLoaError::Credentials(ref err) => err.description(),
            DescribeInterconnectLoaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInterconnectLoaError::ParseError(ref cause) => cause,
            DescribeInterconnectLoaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeInterconnects
#[derive(Debug, PartialEq)]
pub enum DescribeInterconnectsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeInterconnectsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeInterconnectsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeInterconnectsError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeInterconnectsError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeInterconnectsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeInterconnectsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeInterconnectsError {
    fn from(err: serde_json::error::Error) -> DescribeInterconnectsError {
        DescribeInterconnectsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeInterconnectsError {
    fn from(err: CredentialsError) -> DescribeInterconnectsError {
        DescribeInterconnectsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeInterconnectsError {
    fn from(err: HttpDispatchError) -> DescribeInterconnectsError {
        DescribeInterconnectsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeInterconnectsError {
    fn from(err: io::Error) -> DescribeInterconnectsError {
        DescribeInterconnectsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeInterconnectsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeInterconnectsError {
    fn description(&self) -> &str {
        match *self {
            DescribeInterconnectsError::DirectConnectClient(ref cause) => cause,
            DescribeInterconnectsError::DirectConnectServer(ref cause) => cause,
            DescribeInterconnectsError::Validation(ref cause) => cause,
            DescribeInterconnectsError::Credentials(ref err) => err.description(),
            DescribeInterconnectsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeInterconnectsError::ParseError(ref cause) => cause,
            DescribeInterconnectsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeLags
#[derive(Debug, PartialEq)]
pub enum DescribeLagsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeLagsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeLagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeLagsError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DescribeLagsError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeLagsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeLagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeLagsError {
    fn from(err: serde_json::error::Error) -> DescribeLagsError {
        DescribeLagsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLagsError {
    fn from(err: CredentialsError) -> DescribeLagsError {
        DescribeLagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLagsError {
    fn from(err: HttpDispatchError) -> DescribeLagsError {
        DescribeLagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLagsError {
    fn from(err: io::Error) -> DescribeLagsError {
        DescribeLagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLagsError::DirectConnectClient(ref cause) => cause,
            DescribeLagsError::DirectConnectServer(ref cause) => cause,
            DescribeLagsError::Validation(ref cause) => cause,
            DescribeLagsError::Credentials(ref err) => err.description(),
            DescribeLagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeLagsError::ParseError(ref cause) => cause,
            DescribeLagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeLoa
#[derive(Debug, PartialEq)]
pub enum DescribeLoaError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeLoaError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeLoaError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeLoaError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DescribeLoaError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeLoaError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeLoaError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeLoaError {
    fn from(err: serde_json::error::Error) -> DescribeLoaError {
        DescribeLoaError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLoaError {
    fn from(err: CredentialsError) -> DescribeLoaError {
        DescribeLoaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoaError {
    fn from(err: HttpDispatchError) -> DescribeLoaError {
        DescribeLoaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoaError {
    fn from(err: io::Error) -> DescribeLoaError {
        DescribeLoaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLoaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoaError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoaError::DirectConnectClient(ref cause) => cause,
            DescribeLoaError::DirectConnectServer(ref cause) => cause,
            DescribeLoaError::Validation(ref cause) => cause,
            DescribeLoaError::Credentials(ref err) => err.description(),
            DescribeLoaError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeLoaError::ParseError(ref cause) => cause,
            DescribeLoaError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeLocations
#[derive(Debug, PartialEq)]
pub enum DescribeLocationsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeLocationsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeLocationsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeLocationsError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DescribeLocationsError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeLocationsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeLocationsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeLocationsError {
    fn from(err: serde_json::error::Error) -> DescribeLocationsError {
        DescribeLocationsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLocationsError {
    fn from(err: CredentialsError) -> DescribeLocationsError {
        DescribeLocationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLocationsError {
    fn from(err: HttpDispatchError) -> DescribeLocationsError {
        DescribeLocationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLocationsError {
    fn from(err: io::Error) -> DescribeLocationsError {
        DescribeLocationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeLocationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLocationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLocationsError::DirectConnectClient(ref cause) => cause,
            DescribeLocationsError::DirectConnectServer(ref cause) => cause,
            DescribeLocationsError::Validation(ref cause) => cause,
            DescribeLocationsError::Credentials(ref err) => err.description(),
            DescribeLocationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLocationsError::ParseError(ref cause) => cause,
            DescribeLocationsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeTagsError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return DescribeTagsError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeTagsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeTagsError {
    fn from(err: serde_json::error::Error) -> DescribeTagsError {
        DescribeTagsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTagsError {
    fn from(err: CredentialsError) -> DescribeTagsError {
        DescribeTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagsError {
    fn from(err: HttpDispatchError) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagsError {
    fn from(err: io::Error) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::DirectConnectClient(ref cause) => cause,
            DescribeTagsError::DirectConnectServer(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::ParseError(ref cause) => cause,
            DescribeTagsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeVirtualGateways
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualGatewaysError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeVirtualGatewaysError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeVirtualGatewaysError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeVirtualGatewaysError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeVirtualGatewaysError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeVirtualGatewaysError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeVirtualGatewaysError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeVirtualGatewaysError {
    fn from(err: serde_json::error::Error) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeVirtualGatewaysError {
    fn from(err: CredentialsError) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeVirtualGatewaysError {
    fn from(err: HttpDispatchError) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeVirtualGatewaysError {
    fn from(err: io::Error) -> DescribeVirtualGatewaysError {
        DescribeVirtualGatewaysError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeVirtualGatewaysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVirtualGatewaysError {
    fn description(&self) -> &str {
        match *self {
            DescribeVirtualGatewaysError::DirectConnectClient(ref cause) => cause,
            DescribeVirtualGatewaysError::DirectConnectServer(ref cause) => cause,
            DescribeVirtualGatewaysError::Validation(ref cause) => cause,
            DescribeVirtualGatewaysError::Credentials(ref err) => err.description(),
            DescribeVirtualGatewaysError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeVirtualGatewaysError::ParseError(ref cause) => cause,
            DescribeVirtualGatewaysError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeVirtualInterfaces
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualInterfacesError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DescribeVirtualInterfacesError {
    pub fn from_response(res: BufferedHttpResponse) -> DescribeVirtualInterfacesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DescribeVirtualInterfacesError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DescribeVirtualInterfacesError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeVirtualInterfacesError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeVirtualInterfacesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeVirtualInterfacesError {
    fn from(err: serde_json::error::Error) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeVirtualInterfacesError {
    fn from(err: CredentialsError) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeVirtualInterfacesError {
    fn from(err: HttpDispatchError) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeVirtualInterfacesError {
    fn from(err: io::Error) -> DescribeVirtualInterfacesError {
        DescribeVirtualInterfacesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeVirtualInterfacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVirtualInterfacesError {
    fn description(&self) -> &str {
        match *self {
            DescribeVirtualInterfacesError::DirectConnectClient(ref cause) => cause,
            DescribeVirtualInterfacesError::DirectConnectServer(ref cause) => cause,
            DescribeVirtualInterfacesError::Validation(ref cause) => cause,
            DescribeVirtualInterfacesError::Credentials(ref err) => err.description(),
            DescribeVirtualInterfacesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeVirtualInterfacesError::ParseError(ref cause) => cause,
            DescribeVirtualInterfacesError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateConnectionFromLag
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectionFromLagError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DisassociateConnectionFromLagError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateConnectionFromLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return DisassociateConnectionFromLagError::DirectConnectClient(String::from(
                        error_message,
                    ))
                }
                "DirectConnectServerException" => {
                    return DisassociateConnectionFromLagError::DirectConnectServer(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DisassociateConnectionFromLagError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DisassociateConnectionFromLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateConnectionFromLagError {
    fn from(err: serde_json::error::Error) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateConnectionFromLagError {
    fn from(err: CredentialsError) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateConnectionFromLagError {
    fn from(err: HttpDispatchError) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateConnectionFromLagError {
    fn from(err: io::Error) -> DisassociateConnectionFromLagError {
        DisassociateConnectionFromLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateConnectionFromLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateConnectionFromLagError {
    fn description(&self) -> &str {
        match *self {
            DisassociateConnectionFromLagError::DirectConnectClient(ref cause) => cause,
            DisassociateConnectionFromLagError::DirectConnectServer(ref cause) => cause,
            DisassociateConnectionFromLagError::Validation(ref cause) => cause,
            DisassociateConnectionFromLagError::Credentials(ref err) => err.description(),
            DisassociateConnectionFromLagError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateConnectionFromLagError::ParseError(ref cause) => cause,
            DisassociateConnectionFromLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// <p>A tag key was specified more than once.</p>
    DuplicateTagKeys(String),
    /// <p>You have reached the limit on the number of tags that can be assigned to a Direct Connect resource.</p>
    TooManyTags(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> TagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return TagResourceError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return TagResourceError::DirectConnectServer(String::from(error_message))
                }
                "DuplicateTagKeysException" => {
                    return TagResourceError::DuplicateTagKeys(String::from(error_message))
                }
                "TooManyTagsException" => {
                    return TagResourceError::TooManyTags(String::from(error_message))
                }
                "ValidationException" => {
                    return TagResourceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return TagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::DirectConnectClient(ref cause) => cause,
            TagResourceError::DirectConnectServer(ref cause) => cause,
            TagResourceError::DuplicateTagKeys(ref cause) => cause,
            TagResourceError::TooManyTags(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::ParseError(ref cause) => cause,
            TagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> UntagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return UntagResourceError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return UntagResourceError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return UntagResourceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UntagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::DirectConnectClient(ref cause) => cause,
            UntagResourceError::DirectConnectServer(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateLag
#[derive(Debug, PartialEq)]
pub enum UpdateLagError {
    /// <p>The API was called with invalid parameters. The error message will contain additional details about the cause.</p>
    DirectConnectClient(String),
    /// <p>A server-side error occurred during the API call. The error message will contain additional details about the cause.</p>
    DirectConnectServer(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateLagError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateLagError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "DirectConnectClientException" => {
                    return UpdateLagError::DirectConnectClient(String::from(error_message))
                }
                "DirectConnectServerException" => {
                    return UpdateLagError::DirectConnectServer(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateLagError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateLagError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateLagError {
    fn from(err: serde_json::error::Error) -> UpdateLagError {
        UpdateLagError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateLagError {
    fn from(err: CredentialsError) -> UpdateLagError {
        UpdateLagError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateLagError {
    fn from(err: HttpDispatchError) -> UpdateLagError {
        UpdateLagError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateLagError {
    fn from(err: io::Error) -> UpdateLagError {
        UpdateLagError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateLagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLagError {
    fn description(&self) -> &str {
        match *self {
            UpdateLagError::DirectConnectClient(ref cause) => cause,
            UpdateLagError::DirectConnectServer(ref cause) => cause,
            UpdateLagError::Validation(ref cause) => cause,
            UpdateLagError::Credentials(ref err) => err.description(),
            UpdateLagError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateLagError::ParseError(ref cause) => cause,
            UpdateLagError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Direct Connect API. AWS Direct Connect clients implement this trait.
pub trait DirectConnect {
    /// <p><p>Deprecated in favor of <a>AllocateHostedConnection</a>.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_connection_on_interconnect(
        &self,
        input: AllocateConnectionOnInterconnectRequest,
    ) -> RusotoFuture<Connection, AllocateConnectionOnInterconnectError>;

    /// <p><p>Creates a hosted connection on an interconnect or a link aggregation group (LAG).</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect or LAG.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_hosted_connection(
        &self,
        input: AllocateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AllocateHostedConnectionError>;

    /// <p>Provisions a private virtual interface to be owned by another AWS customer.</p> <p>Virtual interfaces created using this action must be confirmed by the virtual interface owner by using the <a>ConfirmPrivateVirtualInterface</a> action. Until then, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p>
    fn allocate_private_virtual_interface(
        &self,
        input: AllocatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePrivateVirtualInterfaceError>;

    /// <p>Provisions a public virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a public virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPublicVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>
    fn allocate_public_virtual_interface(
        &self,
        input: AllocatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePublicVirtualInterfaceError>;

    /// <p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS will be interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can reassociate a connection that's currently associated with a different LAG; however, if removing the connection will cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>
    fn associate_connection_with_lag(
        &self,
        input: AssociateConnectionWithLagRequest,
    ) -> RusotoFuture<Connection, AssociateConnectionWithLagError>;

    /// <p><p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection&#39;s connectivity to AWS as it is being migrated.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn associate_hosted_connection(
        &self,
        input: AssociateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AssociateHostedConnectionError>;

    /// <p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails. </p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>In order to reassociate a virtual interface to a new connection or LAG, the requester must own either the virtual interface itself or the connection to which the virtual interface is currently associated. Additionally, the requester must own the connection or LAG to which the virtual interface will be newly associated.</p>
    fn associate_virtual_interface(
        &self,
        input: AssociateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AssociateVirtualInterfaceError>;

    /// <p>Confirm the creation of a hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the 'Ordering' state, and will remain in this state until the owner calls ConfirmConnection to confirm creation of the hosted connection.</p>
    fn confirm_connection(
        &self,
        input: ConfirmConnectionRequest,
    ) -> RusotoFuture<ConfirmConnectionResponse, ConfirmConnectionError>;

    /// <p>Accept ownership of a private virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the virtual interface will be created and attached to the given virtual private gateway or direct connect gateway, and will be available for handling traffic.</p>
    fn confirm_private_virtual_interface(
        &self,
        input: ConfirmPrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError>;

    /// <p>Accept ownership of a public virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the specified virtual interface will be created and made available for handling traffic.</p>
    fn confirm_public_virtual_interface(
        &self,
        input: ConfirmPublicVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError>;

    /// <p>Creates a new BGP peer on a specified virtual interface. The BGP peer cannot be in the same address family (IPv4/IPv6) of an existing BGP peer on the virtual interface.</p> <p>You must create a BGP peer for the corresponding address family in order to access AWS resources that also use that address family.</p> <p>When creating a IPv6 BGP peer, the Amazon address and customer address fields must be left blank. IPv6 addresses are automatically assigned from Amazon's pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>
    fn create_bgp_peer(
        &self,
        input: CreateBGPPeerRequest,
    ) -> RusotoFuture<CreateBGPPeerResponse, CreateBGPPeerError>;

    /// <p>Creates a new connection between the customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard 1 gigabit or 10 gigabit Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router. An AWS Direct Connect location provides access to Amazon Web Services in the region it is associated with. You can establish connections with AWS Direct Connect locations in multiple regions, but a connection in one region does not provide connectivity to other regions.</p> <p>To find the locations for your region, use <a>DescribeLocations</a>.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection will be created.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<Connection, CreateConnectionError>;

    /// <p>Creates a new direct connect gateway. A direct connect gateway is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. direct connect gateways are global and visible in any AWS region after they are created. The virtual interfaces and virtual private gateways that are connected through a direct connect gateway can be in different regions. This enables you to connect to a VPC in any region, regardless of the region in which the virtual interfaces are located, and pass traffic between them.</p>
    fn create_direct_connect_gateway(
        &self,
        input: CreateDirectConnectGatewayRequest,
    ) -> RusotoFuture<CreateDirectConnectGatewayResult, CreateDirectConnectGatewayError>;

    /// <p>Creates an association between a direct connect gateway and a virtual private gateway (VGW). The VGW must be attached to a VPC and must not be associated with another direct connect gateway.</p>
    fn create_direct_connect_gateway_association(
        &self,
        input: CreateDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        CreateDirectConnectGatewayAssociationResult,
        CreateDirectConnectGatewayAssociationError,
    >;

    /// <p><p>Creates a new interconnect between a AWS Direct Connect partner&#39;s network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the AWS Direct Connect partner&#39;s network to an AWS Direct Connect location over a standard 1 Gbps or 10 Gbps Ethernet fiber-optic cable. One end is connected to the partner&#39;s router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect will be created.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling AllocateConnectionOnInterconnect. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect partner.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn create_interconnect(
        &self,
        input: CreateInterconnectRequest,
    ) -> RusotoFuture<Interconnect, CreateInterconnectError>;

    /// <p>Creates a new link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple 1 gigabit or 10 gigabit interfaces, allowing you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth (for example, 10 Gbps), and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>
    fn create_lag(&self, input: CreateLagRequest) -> RusotoFuture<Lag, CreateLagError>;

    /// <p>Creates a new private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface supports sending traffic to a single virtual private cloud (VPC).</p>
    fn create_private_virtual_interface(
        &self,
        input: CreatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePrivateVirtualInterfaceError>;

    /// <p>Creates a new public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon Simple Storage Service (Amazon S3).</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>
    fn create_public_virtual_interface(
        &self,
        input: CreatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePublicVirtualInterfaceError>;

    /// <p>Deletes a BGP peer on the specified virtual interface that matches the specified customer address and ASN. You cannot delete the last BGP peer from a virtual interface.</p>
    fn delete_bgp_peer(
        &self,
        input: DeleteBGPPeerRequest,
    ) -> RusotoFuture<DeleteBGPPeerResponse, DeleteBGPPeerError>;

    /// <p>Deletes the connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. You need to cancel separately with the providers any services or charges for cross-connects or network circuits that connect you to the AWS Direct Connect location.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<Connection, DeleteConnectionError>;

    /// <p>Deletes a direct connect gateway. You must first delete all virtual interfaces that are attached to the direct connect gateway and disassociate all virtual private gateways that are associated with the direct connect gateway.</p>
    fn delete_direct_connect_gateway(
        &self,
        input: DeleteDirectConnectGatewayRequest,
    ) -> RusotoFuture<DeleteDirectConnectGatewayResult, DeleteDirectConnectGatewayError>;

    /// <p>Deletes the association between a direct connect gateway and a virtual private gateway.</p>
    fn delete_direct_connect_gateway_association(
        &self,
        input: DeleteDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        DeleteDirectConnectGatewayAssociationResult,
        DeleteDirectConnectGatewayAssociationError,
    >;

    /// <p><p>Deletes the specified interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn delete_interconnect(
        &self,
        input: DeleteInterconnectRequest,
    ) -> RusotoFuture<DeleteInterconnectResponse, DeleteInterconnectError>;

    /// <p>Deletes a link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>
    fn delete_lag(&self, input: DeleteLagRequest) -> RusotoFuture<Lag, DeleteLagError>;

    /// <p>Deletes a virtual interface.</p>
    fn delete_virtual_interface(
        &self,
        input: DeleteVirtualInterfaceRequest,
    ) -> RusotoFuture<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError>;

    /// <p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for a Connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>
    fn describe_connection_loa(
        &self,
        input: DescribeConnectionLoaRequest,
    ) -> RusotoFuture<DescribeConnectionLoaResponse, DescribeConnectionLoaError>;

    /// <p>Displays all connections in this region.</p> <p>If a connection ID is provided, the call returns only that particular connection.</p>
    fn describe_connections(
        &self,
        input: DescribeConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsError>;

    /// <p><p>Deprecated in favor of <a>DescribeHostedConnections</a>.</p> <p>Returns a list of connections that have been provisioned on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_connections_on_interconnect(
        &self,
        input: DescribeConnectionsOnInterconnectRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsOnInterconnectError>;

    /// <p>Returns a list of all direct connect gateway and virtual private gateway (VGW) associations. Either a direct connect gateway ID or a VGW ID must be provided in the request. If a direct connect gateway ID is provided, the response returns all VGWs associated with the direct connect gateway. If a VGW ID is provided, the response returns all direct connect gateways associated with the VGW. If both are provided, the response only returns the association that matches both the direct connect gateway and the VGW.</p>
    fn describe_direct_connect_gateway_associations(
        &self,
        input: DescribeDirectConnectGatewayAssociationsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAssociationsResult,
        DescribeDirectConnectGatewayAssociationsError,
    >;

    /// <p>Returns a list of all direct connect gateway and virtual interface (VIF) attachments. Either a direct connect gateway ID or a VIF ID must be provided in the request. If a direct connect gateway ID is provided, the response returns all VIFs attached to the direct connect gateway. If a VIF ID is provided, the response returns all direct connect gateways attached to the VIF. If both are provided, the response only returns the attachment that matches both the direct connect gateway and the VIF.</p>
    fn describe_direct_connect_gateway_attachments(
        &self,
        input: DescribeDirectConnectGatewayAttachmentsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAttachmentsResult,
        DescribeDirectConnectGatewayAttachmentsError,
    >;

    /// <p>Returns a list of direct connect gateways in your account. Deleted direct connect gateways are not returned. You can provide a direct connect gateway ID in the request to return information about the specific direct connect gateway only. Otherwise, if a direct connect gateway ID is not provided, information about all of your direct connect gateways is returned. </p>
    fn describe_direct_connect_gateways(
        &self,
        input: DescribeDirectConnectGatewaysRequest,
    ) -> RusotoFuture<DescribeDirectConnectGatewaysResult, DescribeDirectConnectGatewaysError>;

    /// <p><p>Returns a list of hosted connections that have been provisioned on the given interconnect or link aggregation group (LAG).</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_hosted_connections(
        &self,
        input: DescribeHostedConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeHostedConnectionsError>;

    /// <p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for an Interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>
    fn describe_interconnect_loa(
        &self,
        input: DescribeInterconnectLoaRequest,
    ) -> RusotoFuture<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError>;

    /// <p>Returns a list of interconnects owned by the AWS account.</p> <p>If an interconnect ID is provided, it will only return this particular interconnect.</p>
    fn describe_interconnects(
        &self,
        input: DescribeInterconnectsRequest,
    ) -> RusotoFuture<Interconnects, DescribeInterconnectsError>;

    /// <p>Describes the link aggregation groups (LAGs) in your account. </p> <p>If a LAG ID is provided, only information about the specified LAG is returned.</p>
    fn describe_lags(&self, input: DescribeLagsRequest) -> RusotoFuture<Lags, DescribeLagsError>;

    /// <p>Returns the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>
    fn describe_loa(&self, input: DescribeLoaRequest) -> RusotoFuture<Loa, DescribeLoaError>;

    /// <p>Returns the list of AWS Direct Connect locations in the current AWS region. These are the locations that may be selected when calling <a>CreateConnection</a> or <a>CreateInterconnect</a>.</p>
    fn describe_locations(&self) -> RusotoFuture<Locations, DescribeLocationsError>;

    /// <p>Describes the tags associated with the specified Direct Connect resources.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError>;

    /// <p>Returns a list of virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linking to a virtual private gateway. A virtual private gateway can be managed via Amazon Virtual Private Cloud (VPC) console or the <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html">EC2 CreateVpnGateway</a> action.</p>
    fn describe_virtual_gateways(
        &self,
    ) -> RusotoFuture<VirtualGateways, DescribeVirtualGatewaysError>;

    /// <p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p>
    fn describe_virtual_interfaces(
        &self,
        input: DescribeVirtualInterfacesRequest,
    ) -> RusotoFuture<VirtualInterfaces, DescribeVirtualInterfacesError>;

    /// <p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect partner is automatically converted to an interconnect.</p> <p>If disassociating the connection will cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>
    fn disassociate_connection_from_lag(
        &self,
        input: DisassociateConnectionFromLagRequest,
    ) -> RusotoFuture<Connection, DisassociateConnectionFromLagError>;

    /// <p>Adds the specified tags to the specified Direct Connect resource. Each Direct Connect resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the Direct Connect resource, this action updates its value.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes one or more tags from the specified Direct Connect resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates the attributes of a link aggregation group (LAG). </p> <p>You can update the following attributes: </p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value, and the number of operational connections falls below the specified value, the LAG will automatically go down to avoid overutilization of the remaining connections. Adjusting this value should be done with care as it could force the LAG down if the value is set higher than the current number of operational connections.</p>
    fn update_lag(&self, input: UpdateLagRequest) -> RusotoFuture<Lag, UpdateLagError>;
}
/// A client for the AWS Direct Connect API.
pub struct DirectConnectClient {
    client: Client,
    region: region::Region,
}

impl DirectConnectClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DirectConnectClient {
        DirectConnectClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DirectConnectClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        DirectConnectClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl DirectConnect for DirectConnectClient {
    /// <p><p>Deprecated in favor of <a>AllocateHostedConnection</a>.</p> <p>Creates a hosted connection on an interconnect.</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_connection_on_interconnect(
        &self,
        input: AllocateConnectionOnInterconnectRequest,
    ) -> RusotoFuture<Connection, AllocateConnectionOnInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocateConnectionOnInterconnect",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocateConnectionOnInterconnectError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Creates a hosted connection on an interconnect or a link aggregation group (LAG).</p> <p>Allocates a VLAN number and a specified amount of bandwidth for use by a hosted connection on the given interconnect or LAG.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn allocate_hosted_connection(
        &self,
        input: AllocateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AllocateHostedConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AllocateHostedConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocateHostedConnectionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Provisions a private virtual interface to be owned by another AWS customer.</p> <p>Virtual interfaces created using this action must be confirmed by the virtual interface owner by using the <a>ConfirmPrivateVirtualInterface</a> action. Until then, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p>
    fn allocate_private_virtual_interface(
        &self,
        input: AllocatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePrivateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocatePrivateVirtualInterface",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocatePrivateVirtualInterfaceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Provisions a public virtual interface to be owned by a different customer.</p> <p>The owner of a connection calls this function to provision a public virtual interface which will be owned by another AWS customer.</p> <p>Virtual interfaces created using this function must be confirmed by the virtual interface owner by calling ConfirmPublicVirtualInterface. Until this step has been completed, the virtual interface will be in 'Confirming' state, and will not be available for handling traffic.</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>
    fn allocate_public_virtual_interface(
        &self,
        input: AllocatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AllocatePublicVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.AllocatePublicVirtualInterface",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AllocatePublicVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates an existing connection with a link aggregation group (LAG). The connection is interrupted and re-established as a member of the LAG (connectivity to AWS will be interrupted). The connection must be hosted on the same AWS Direct Connect endpoint as the LAG, and its bandwidth must match the bandwidth for the LAG. You can reassociate a connection that's currently associated with a different LAG; however, if removing the connection will cause the original LAG to fall below its setting for minimum number of operational connections, the request fails.</p> <p>Any virtual interfaces that are directly associated with the connection are automatically re-associated with the LAG. If the connection was originally associated with a different LAG, the virtual interfaces remain associated with the original LAG.</p> <p>For interconnects, any hosted connections are automatically re-associated with the LAG. If the interconnect was originally associated with a different LAG, the hosted connections remain associated with the original LAG.</p>
    fn associate_connection_with_lag(
        &self,
        input: AssociateConnectionWithLagRequest,
    ) -> RusotoFuture<Connection, AssociateConnectionWithLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateConnectionWithLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateConnectionWithLagError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Associates a hosted connection and its virtual interfaces with a link aggregation group (LAG) or interconnect. If the target interconnect or LAG has an existing hosted connection with a conflicting VLAN number or IP address, the operation fails. This action temporarily interrupts the hosted connection&#39;s connectivity to AWS as it is being migrated.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn associate_hosted_connection(
        &self,
        input: AssociateHostedConnectionRequest,
    ) -> RusotoFuture<Connection, AssociateHostedConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateHostedConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateHostedConnectionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates a virtual interface with a specified link aggregation group (LAG) or connection. Connectivity to AWS is temporarily interrupted as the virtual interface is being migrated. If the target connection or LAG has an associated virtual interface with a conflicting VLAN number or a conflicting IP address, the operation fails. </p> <p>Virtual interfaces associated with a hosted connection cannot be associated with a LAG; hosted connections must be migrated along with their virtual interfaces using <a>AssociateHostedConnection</a>.</p> <p>In order to reassociate a virtual interface to a new connection or LAG, the requester must own either the virtual interface itself or the connection to which the virtual interface is currently associated. Additionally, the requester must own the connection or LAG to which the virtual interface will be newly associated.</p>
    fn associate_virtual_interface(
        &self,
        input: AssociateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, AssociateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.AssociateVirtualInterface");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AssociateVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Confirm the creation of a hosted connection on an interconnect.</p> <p>Upon creation, the hosted connection is initially in the 'Ordering' state, and will remain in this state until the owner calls ConfirmConnection to confirm creation of the hosted connection.</p>
    fn confirm_connection(
        &self,
        input: ConfirmConnectionRequest,
    ) -> RusotoFuture<ConfirmConnectionResponse, ConfirmConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.ConfirmConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ConfirmConnectionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ConfirmConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Accept ownership of a private virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the virtual interface will be created and attached to the given virtual private gateway or direct connect gateway, and will be available for handling traffic.</p>
    fn confirm_private_virtual_interface(
        &self,
        input: ConfirmPrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPrivateVirtualInterfaceResponse, ConfirmPrivateVirtualInterfaceError>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.ConfirmPrivateVirtualInterface",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ConfirmPrivateVirtualInterfaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ConfirmPrivateVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Accept ownership of a public virtual interface created by another customer.</p> <p>After the virtual interface owner calls this function, the specified virtual interface will be created and made available for handling traffic.</p>
    fn confirm_public_virtual_interface(
        &self,
        input: ConfirmPublicVirtualInterfaceRequest,
    ) -> RusotoFuture<ConfirmPublicVirtualInterfaceResponse, ConfirmPublicVirtualInterfaceError>
    {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.ConfirmPublicVirtualInterface",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ConfirmPublicVirtualInterfaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ConfirmPublicVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new BGP peer on a specified virtual interface. The BGP peer cannot be in the same address family (IPv4/IPv6) of an existing BGP peer on the virtual interface.</p> <p>You must create a BGP peer for the corresponding address family in order to access AWS resources that also use that address family.</p> <p>When creating a IPv6 BGP peer, the Amazon address and customer address fields must be left blank. IPv6 addresses are automatically assigned from Amazon's pool of IPv6 addresses; you cannot specify custom IPv6 addresses.</p> <p>For a public virtual interface, the Autonomous System Number (ASN) must be private or already whitelisted for the virtual interface.</p>
    fn create_bgp_peer(
        &self,
        input: CreateBGPPeerRequest,
    ) -> RusotoFuture<CreateBGPPeerResponse, CreateBGPPeerError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateBGPPeer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateBGPPeerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateBGPPeerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new connection between the customer network and a specific AWS Direct Connect location.</p> <p>A connection links your internal network to an AWS Direct Connect location over a standard 1 gigabit or 10 gigabit Ethernet fiber-optic cable. One end of the cable is connected to your router, the other to an AWS Direct Connect router. An AWS Direct Connect location provides access to Amazon Web Services in the region it is associated with. You can establish connections with AWS Direct Connect locations in multiple regions, but a connection in one region does not provide connectivity to other regions.</p> <p>To find the locations for your region, use <a>DescribeLocations</a>.</p> <p>You can automatically add the new connection to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new connection is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no connection will be created.</p>
    fn create_connection(
        &self,
        input: CreateConnectionRequest,
    ) -> RusotoFuture<Connection, CreateConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new direct connect gateway. A direct connect gateway is an intermediate object that enables you to connect a set of virtual interfaces and virtual private gateways. direct connect gateways are global and visible in any AWS region after they are created. The virtual interfaces and virtual private gateways that are connected through a direct connect gateway can be in different regions. This enables you to connect to a VPC in any region, regardless of the region in which the virtual interfaces are located, and pass traffic between them.</p>
    fn create_direct_connect_gateway(
        &self,
        input: CreateDirectConnectGatewayRequest,
    ) -> RusotoFuture<CreateDirectConnectGatewayResult, CreateDirectConnectGatewayError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateDirectConnectGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDirectConnectGatewayResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDirectConnectGatewayError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an association between a direct connect gateway and a virtual private gateway (VGW). The VGW must be attached to a VPC and must not be associated with another direct connect gateway.</p>
    fn create_direct_connect_gateway_association(
        &self,
        input: CreateDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        CreateDirectConnectGatewayAssociationResult,
        CreateDirectConnectGatewayAssociationError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreateDirectConnectGatewayAssociation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDirectConnectGatewayAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDirectConnectGatewayAssociationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Creates a new interconnect between a AWS Direct Connect partner&#39;s network and a specific AWS Direct Connect location.</p> <p>An interconnect is a connection which is capable of hosting other connections. The AWS Direct Connect partner can use an interconnect to provide sub-1Gbps AWS Direct Connect service to tier 2 customers who do not have their own connections. Like a standard connection, an interconnect links the AWS Direct Connect partner&#39;s network to an AWS Direct Connect location over a standard 1 Gbps or 10 Gbps Ethernet fiber-optic cable. One end is connected to the partner&#39;s router, the other to an AWS Direct Connect router.</p> <p>You can automatically add the new interconnect to a link aggregation group (LAG) by specifying a LAG ID in the request. This ensures that the new interconnect is allocated on the same AWS Direct Connect endpoint that hosts the specified LAG. If there are no available ports on the endpoint, the request fails and no interconnect will be created.</p> <p>For each end customer, the AWS Direct Connect partner provisions a connection on their interconnect by calling AllocateConnectionOnInterconnect. The end customer can then connect to AWS resources by creating a virtual interface on their connection, using the VLAN assigned to them by the AWS Direct Connect partner.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn create_interconnect(
        &self,
        input: CreateInterconnectRequest,
    ) -> RusotoFuture<Interconnect, CreateInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateInterconnect");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Interconnect>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateInterconnectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new link aggregation group (LAG) with the specified number of bundled physical connections between the customer network and a specific AWS Direct Connect location. A LAG is a logical interface that uses the Link Aggregation Control Protocol (LACP) to aggregate multiple 1 gigabit or 10 gigabit interfaces, allowing you to treat them as a single interface.</p> <p>All connections in a LAG must use the same bandwidth (for example, 10 Gbps), and must terminate at the same AWS Direct Connect endpoint.</p> <p>You can have up to 10 connections per LAG. Regardless of this limit, if you request more connections for the LAG than AWS Direct Connect can allocate on a single endpoint, no LAG is created.</p> <p>You can specify an existing physical connection or interconnect to include in the LAG (which counts towards the total number of connections). Doing so interrupts the current physical connection or hosted connections, and re-establishes them as a member of the LAG. The LAG will be created on the same AWS Direct Connect endpoint to which the connection terminates. Any virtual interfaces associated with the connection are automatically disassociated and re-associated with the LAG. The connection ID does not change.</p> <p>If the AWS account used to create a LAG is a registered AWS Direct Connect partner, the LAG is automatically enabled to host sub-connections. For a LAG owned by a partner, any associated virtual interfaces cannot be directly configured.</p>
    fn create_lag(&self, input: CreateLagRequest) -> RusotoFuture<Lag, CreateLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.CreateLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lag>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateLagError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new private virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A private virtual interface supports sending traffic to a single virtual private cloud (VPC).</p>
    fn create_private_virtual_interface(
        &self,
        input: CreatePrivateVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePrivateVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreatePrivateVirtualInterface",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreatePrivateVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new public virtual interface. A virtual interface is the VLAN that transports AWS Direct Connect traffic. A public virtual interface supports sending traffic to public services of AWS such as Amazon Simple Storage Service (Amazon S3).</p> <p>When creating an IPv6 public virtual interface (addressFamily is 'ipv6'), the customer and amazon address fields should be left blank to use auto-assigned IPv6 space. Custom IPv6 Addresses are currently not supported.</p>
    fn create_public_virtual_interface(
        &self,
        input: CreatePublicVirtualInterfaceRequest,
    ) -> RusotoFuture<VirtualInterface, CreatePublicVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.CreatePublicVirtualInterface",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterface>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreatePublicVirtualInterfaceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a BGP peer on the specified virtual interface that matches the specified customer address and ASN. You cannot delete the last BGP peer from a virtual interface.</p>
    fn delete_bgp_peer(
        &self,
        input: DeleteBGPPeerRequest,
    ) -> RusotoFuture<DeleteBGPPeerResponse, DeleteBGPPeerError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteBGPPeer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteBGPPeerResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteBGPPeerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the connection.</p> <p>Deleting a connection only stops the AWS Direct Connect port hour and data transfer charges. You need to cancel separately with the providers any services or charges for cross-connects or network circuits that connect you to the AWS Direct Connect location.</p>
    fn delete_connection(
        &self,
        input: DeleteConnectionRequest,
    ) -> RusotoFuture<Connection, DeleteConnectionError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteConnectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a direct connect gateway. You must first delete all virtual interfaces that are attached to the direct connect gateway and disassociate all virtual private gateways that are associated with the direct connect gateway.</p>
    fn delete_direct_connect_gateway(
        &self,
        input: DeleteDirectConnectGatewayRequest,
    ) -> RusotoFuture<DeleteDirectConnectGatewayResult, DeleteDirectConnectGatewayError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteDirectConnectGateway");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDirectConnectGatewayResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDirectConnectGatewayError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the association between a direct connect gateway and a virtual private gateway.</p>
    fn delete_direct_connect_gateway_association(
        &self,
        input: DeleteDirectConnectGatewayAssociationRequest,
    ) -> RusotoFuture<
        DeleteDirectConnectGatewayAssociationResult,
        DeleteDirectConnectGatewayAssociationError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DeleteDirectConnectGatewayAssociation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDirectConnectGatewayAssociationResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDirectConnectGatewayAssociationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Deletes the specified interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn delete_interconnect(
        &self,
        input: DeleteInterconnectRequest,
    ) -> RusotoFuture<DeleteInterconnectResponse, DeleteInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteInterconnect");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteInterconnectResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteInterconnectError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a link aggregation group (LAG). You cannot delete a LAG if it has active virtual interfaces or hosted connections.</p>
    fn delete_lag(&self, input: DeleteLagRequest) -> RusotoFuture<Lag, DeleteLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lag>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteLagError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a virtual interface.</p>
    fn delete_virtual_interface(
        &self,
        input: DeleteVirtualInterfaceRequest,
    ) -> RusotoFuture<DeleteVirtualInterfaceResponse, DeleteVirtualInterfaceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DeleteVirtualInterface");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteVirtualInterfaceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteVirtualInterfaceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for a Connection.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that your APN partner or service provider uses when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>
    fn describe_connection_loa(
        &self,
        input: DescribeConnectionLoaRequest,
    ) -> RusotoFuture<DescribeConnectionLoaResponse, DescribeConnectionLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnectionLoa");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeConnectionLoaResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeConnectionLoaError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Displays all connections in this region.</p> <p>If a connection ID is provided, the call returns only that particular connection.</p>
    fn describe_connections(
        &self,
        input: DescribeConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connections>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeConnectionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Deprecated in favor of <a>DescribeHostedConnections</a>.</p> <p>Returns a list of connections that have been provisioned on the given interconnect.</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_connections_on_interconnect(
        &self,
        input: DescribeConnectionsOnInterconnectRequest,
    ) -> RusotoFuture<Connections, DescribeConnectionsOnInterconnectError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeConnectionsOnInterconnect",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connections>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConnectionsOnInterconnectError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of all direct connect gateway and virtual private gateway (VGW) associations. Either a direct connect gateway ID or a VGW ID must be provided in the request. If a direct connect gateway ID is provided, the response returns all VGWs associated with the direct connect gateway. If a VGW ID is provided, the response returns all direct connect gateways associated with the VGW. If both are provided, the response only returns the association that matches both the direct connect gateway and the VGW.</p>
    fn describe_direct_connect_gateway_associations(
        &self,
        input: DescribeDirectConnectGatewayAssociationsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAssociationsResult,
        DescribeDirectConnectGatewayAssociationsError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGatewayAssociations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDirectConnectGatewayAssociationsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDirectConnectGatewayAssociationsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of all direct connect gateway and virtual interface (VIF) attachments. Either a direct connect gateway ID or a VIF ID must be provided in the request. If a direct connect gateway ID is provided, the response returns all VIFs attached to the direct connect gateway. If a VIF ID is provided, the response returns all direct connect gateways attached to the VIF. If both are provided, the response only returns the attachment that matches both the direct connect gateway and the VIF.</p>
    fn describe_direct_connect_gateway_attachments(
        &self,
        input: DescribeDirectConnectGatewayAttachmentsRequest,
    ) -> RusotoFuture<
        DescribeDirectConnectGatewayAttachmentsResult,
        DescribeDirectConnectGatewayAttachmentsError,
    > {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGatewayAttachments",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDirectConnectGatewayAttachmentsResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDirectConnectGatewayAttachmentsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of direct connect gateways in your account. Deleted direct connect gateways are not returned. You can provide a direct connect gateway ID in the request to return information about the specific direct connect gateway only. Otherwise, if a direct connect gateway ID is not provided, information about all of your direct connect gateways is returned. </p>
    fn describe_direct_connect_gateways(
        &self,
        input: DescribeDirectConnectGatewaysRequest,
    ) -> RusotoFuture<DescribeDirectConnectGatewaysResult, DescribeDirectConnectGatewaysError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DescribeDirectConnectGateways",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDirectConnectGatewaysResult>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDirectConnectGatewaysError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Returns a list of hosted connections that have been provisioned on the given interconnect or link aggregation group (LAG).</p> <note> <p>This is intended for use by AWS Direct Connect partners only.</p> </note></p>
    fn describe_hosted_connections(
        &self,
        input: DescribeHostedConnectionsRequest,
    ) -> RusotoFuture<Connections, DescribeHostedConnectionsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeHostedConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connections>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeHostedConnectionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deprecated in favor of <a>DescribeLoa</a>.</p> <p>Returns the LOA-CFA for an Interconnect.</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>
    fn describe_interconnect_loa(
        &self,
        input: DescribeInterconnectLoaRequest,
    ) -> RusotoFuture<DescribeInterconnectLoaResponse, DescribeInterconnectLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnectLoa");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeInterconnectLoaResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeInterconnectLoaError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of interconnects owned by the AWS account.</p> <p>If an interconnect ID is provided, it will only return this particular interconnect.</p>
    fn describe_interconnects(
        &self,
        input: DescribeInterconnectsRequest,
    ) -> RusotoFuture<Interconnects, DescribeInterconnectsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeInterconnects");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Interconnects>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeInterconnectsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the link aggregation groups (LAGs) in your account. </p> <p>If a LAG ID is provided, only information about the specified LAG is returned.</p>
    fn describe_lags(&self, input: DescribeLagsRequest) -> RusotoFuture<Lags, DescribeLagsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lags>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeLagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the LOA-CFA for a connection, interconnect, or link aggregation group (LAG).</p> <p>The Letter of Authorization - Connecting Facility Assignment (LOA-CFA) is a document that is used when establishing your cross connect to AWS at the colocation facility. For more information, see <a href="http://docs.aws.amazon.com/directconnect/latest/UserGuide/Colocation.html">Requesting Cross Connects at AWS Direct Connect Locations</a> in the AWS Direct Connect user guide.</p>
    fn describe_loa(&self, input: DescribeLoaRequest) -> RusotoFuture<Loa, DescribeLoaError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLoa");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Loa>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeLoaError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the list of AWS Direct Connect locations in the current AWS region. These are the locations that may be selected when calling <a>CreateConnection</a> or <a>CreateInterconnect</a>.</p>
    fn describe_locations(&self) -> RusotoFuture<Locations, DescribeLocationsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeLocations");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Locations>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeLocationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the tags associated with the specified Direct Connect resources.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of virtual private gateways owned by the AWS account.</p> <p>You can create one or more AWS Direct Connect private virtual interfaces linking to a virtual private gateway. A virtual private gateway can be managed via Amazon Virtual Private Cloud (VPC) console or the <a href="http://docs.aws.amazon.com/AWSEC2/latest/APIReference/ApiReference-query-CreateVpnGateway.html">EC2 CreateVpnGateway</a> action.</p>
    fn describe_virtual_gateways(
        &self,
    ) -> RusotoFuture<VirtualGateways, DescribeVirtualGatewaysError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualGateways");
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualGateways>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeVirtualGatewaysError::from_response(response))
                }))
            }
        })
    }

    /// <p>Displays all virtual interfaces for an AWS account. Virtual interfaces deleted fewer than 15 minutes before you make the request are also returned. If you specify a connection ID, only the virtual interfaces associated with the connection are returned. If you specify a virtual interface ID, then only a single virtual interface is returned.</p> <p>A virtual interface (VLAN) transmits the traffic between the AWS Direct Connect location and the customer.</p>
    fn describe_virtual_interfaces(
        &self,
        input: DescribeVirtualInterfacesRequest,
    ) -> RusotoFuture<VirtualInterfaces, DescribeVirtualInterfacesError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.DescribeVirtualInterfaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<VirtualInterfaces>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeVirtualInterfacesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Disassociates a connection from a link aggregation group (LAG). The connection is interrupted and re-established as a standalone connection (the connection is not deleted; to delete the connection, use the <a>DeleteConnection</a> request). If the LAG has associated virtual interfaces or hosted connections, they remain associated with the LAG. A disassociated connection owned by an AWS Direct Connect partner is automatically converted to an interconnect.</p> <p>If disassociating the connection will cause the LAG to fall below its setting for minimum number of operational connections, the request fails, except when it's the last member of the LAG. If all connections are disassociated, the LAG continues to exist as an empty LAG with no physical connections. </p>
    fn disassociate_connection_from_lag(
        &self,
        input: DisassociateConnectionFromLagRequest,
    ) -> RusotoFuture<Connection, DisassociateConnectionFromLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "OvertureService.DisassociateConnectionFromLag",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Connection>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DisassociateConnectionFromLagError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds the specified tags to the specified Direct Connect resource. Each Direct Connect resource can have a maximum of 50 tags.</p> <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the Direct Connect resource, this action updates its value.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes one or more tags from the specified Direct Connect resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagResourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the attributes of a link aggregation group (LAG). </p> <p>You can update the following attributes: </p> <ul> <li> <p>The name of the LAG.</p> </li> <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li> </ul> <p>When you create a LAG, the default value for the minimum number of operational connections is zero (0). If you update this value, and the number of operational connections falls below the specified value, the LAG will automatically go down to avoid overutilization of the remaining connections. Adjusting this value should be done with care as it could force the LAG down if the value is set higher than the current number of operational connections.</p>
    fn update_lag(&self, input: UpdateLagRequest) -> RusotoFuture<Lag, UpdateLagError> {
        let mut request = SignedRequest::new("POST", "directconnect", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "OvertureService.UpdateLag");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<Lag>(String::from_utf8_lossy(body.as_ref()).as_ref())
                        .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateLagError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
